#![allow(unknown_lints)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::unreadable_literal,
    clippy::too_many_lines,
    clippy::must_use_candidate
)]

use std::{collections::HashMap, str::Utf8Error};

use ops::{Dim, Op};
use thiserror::Error;

mod ops;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("{0}")]
    Other(String),
    #[error("invalid header")]
    InvalidHeader,
    #[error("invalid bytes in commmand")]
    InvalidOp,
    #[error("invalid id")]
    InvalidId,
    #[error("invalid utf-8 in string")]
    StringFormat(#[from] Utf8Error),
}

pub type SpirvResult<T> = ::std::result::Result<T, Error>;

/// Stores the reflection info of a single SPIRV module.
#[derive(Debug)]
pub struct Module {
    /// Stores information about all type declarations that exist in the given SPIRV module.
    types: HashMap<u32, Type>,
    /// Stores information about all uniform variables that exist in the given SPIRV module.
    uniforms: Vec<Variable>,
    /// Stores information about all push constant variables that exist in the given SPIRV module.
    push_constants: Vec<Variable>,
}

impl Module {
    /// Generates reflection info from a given stream of `words`.
    ///
    /// # Errors
    /// - [`Error::InvalidHeader`] if the SPIRV header is not valid
    /// - [`Error::InvalidOp`] if the binary representation of any instruction in `words` is not valid
    /// - [`Error::InvalidId`] if any type declaration in the SPIRV module reference non-existent IDs
    /// - [`Error::StringFormat`] if any `OpCode` contains a String with invalid UTF-8 characters
    /// - [`Error::Other`] if any other errors occur
    pub fn from_words(mut words: &[u32]) -> SpirvResult<Self> {
        // Check the SPIRV header magic number
        if words.len() < 6 || words[0] != 0x07230203 {
            return Err(Error::InvalidHeader);
        }

        // Skip the rest of the header (Should be parsed in the future)
        words = &words[5..];

        // decode all opcodes
        let mut ops = Vec::new();
        while !words.is_empty() {
            let op = Op::decode(&mut words)?;
            ops.push(op);
        }

        // All OpConstant values are stored in this Vec
        let mut constants = HashMap::new();
        // All type declarations are stored in this Vec
        let mut types = HashMap::new();
        // All variable declarations are stored in this Vec
        let mut vars = HashMap::new();

        Self::collect_types_and_vars(&ops, &mut types, &mut constants, &mut vars)?;
        Self::collect_decorations_and_names(&ops, &mut types, &mut vars);

        // uniforms are all variables that are a pointer with a storage class of Uniform or UniformConstant
        let uniforms = vars
            .iter()
            .filter_map(|(_id, var)| {
                if let Some(Type::Pointer {
                    storage_class: StorageClass::Uniform | StorageClass::UniformConstant,
                    pointed_type_id,
                }) = types.get(&var.type_id)
                {
                    Some(Variable {
                        set: var.set,
                        binding: var.binding,
                        type_id: *pointed_type_id, // for convenience, we store the pointed-to type instead of the pointer, since every uniform is a pointer
                        name: var.name.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        let push_constants = vars
            .iter()
            .filter_map(|(_id, var)| {
                if let Some(Type::Pointer {
                    storage_class: StorageClass::PushConstant,
                    pointed_type_id,
                }) = types.get(&var.type_id)
                {
                    Some(Variable {
                        set: None,
                        binding: None,
                        type_id: *pointed_type_id,
                        name: var.name.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(Self {
            types,
            uniforms,
            push_constants,
        })
    }

    /// Returns the [`Type`] definition indicated by `type_id`, or `None` if `type_id` is not a type.
    pub fn get_type(&self, type_id: u32) -> Option<&Type> {
        self.types.get(&type_id)
    }

    /// Returns all uniform variables declared in the given SPIR-V module.
    pub fn get_uniforms(&self) -> &[Variable] {
        &self.uniforms
    }

    /// Returns all push-constant variables declared in the given SPIR-V module.
    pub fn get_push_constants(&self) -> &[Variable] {
        &self.push_constants
    }

    /// Calculates the size of a primitive type or Struct.
    ///
    /// # Returns
    /// The size of the type indicated by `type_id`
    /// or [`None`] if the size of the given type is not known.
    pub fn get_type_size(&self, type_id: u32) -> Option<u32> {
        if let Some(ty) = self.types.get(&type_id) {
            match ty {
                Type::Int32 | Type::UInt32 | Type::Float32 => Some(4),
                Type::Vec2 => Some(8),
                Type::Vec3 => Some(12),
                Type::Vec4 => Some(16),
                Type::Mat3 => Some(48), // Mat3 works like three Vec3 after another, Vec3 has alignment of Vec4
                Type::Mat4 => Some(64),
                Type::Struct { elements, .. } => {
                    // Since there is no Size Decoration in SPIRV that tells us the size,
                    // we calculate it from the offset of the last member and its size.
                    let last_element = elements.iter().max_by_key(|e| e.offset.unwrap_or(0))?;
                    let offset = last_element.offset?;
                    let size = self.get_type_size(last_element.type_id)?;

                    Some(offset + size)
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Parses all the Op*Decoration and Op*Name instructions
    fn collect_decorations_and_names(
        ops: &[Op],
        types: &mut HashMap<u32, Type>,
        vars: &mut HashMap<u32, Variable>,
    ) {
        for op in ops {
            match op {
                Op::OpName { target, name } => {
                    if let Some(target) = vars.get_mut(&target.0) {
                        target.name = Some(name.clone());
                    } else if let Some(Type::Struct { name: n, .. }) = types.get_mut(&target.0) {
                        *n = Some(name.clone());
                    }
                }
                Op::OpMemberName {
                    target,
                    member_index,
                    name,
                } => {
                    if let Some(Type::Struct { elements, .. }) = types.get_mut(&target.0) {
                        if elements.len() > *member_index as usize {
                            elements[*member_index as usize].name = Some(name.clone());
                        }
                    }
                }
                Op::OpDecorate { target, decoration } => match decoration {
                    ops::Decoration::Binding { binding } => {
                        if let Some(target) = vars.get_mut(&target.0) {
                            target.binding = Some(*binding);
                        }
                    }
                    ops::Decoration::DescriptorSet { set } => {
                        if let Some(target) = vars.get_mut(&target.0) {
                            target.set = Some(*set);
                        }
                    }
                    _ => {}
                },
                Op::OpMemberDecorate {
                    target,
                    member_index,
                    decoration: ops::Decoration::Offset { offset },
                } => {
                    if let Some(Type::Struct { elements, .. }) = types.get_mut(&target.0) {
                        if elements.len() > *member_index as usize {
                            elements[*member_index as usize].offset = Some(*offset);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Parses all the OpType* and OpVariable instructions
    fn collect_types_and_vars(
        ops: &[Op],
        types: &mut HashMap<u32, Type>,
        constants: &mut HashMap<u32, u32>,
        vars: &mut HashMap<u32, Variable>,
    ) -> SpirvResult<()> {
        for op in ops {
            match op {
                Op::OpTypeVoid { result } => {
                    types.insert(result.0, Type::Void);
                }
                Op::OpTypeBool { result } => {
                    types.insert(result.0, Type::Bool);
                }
                Op::OpTypeInt {
                    result,
                    width,
                    signed,
                } => {
                    if *width != 32 {
                        types.insert(result.0, Type::Unknown);
                    } else if *signed == 0 {
                        types.insert(result.0, Type::UInt32);
                    } else {
                        types.insert(result.0, Type::Int32);
                    }
                }
                Op::OpTypeFloat { result, width } => {
                    if *width == 32 {
                        types.insert(result.0, Type::Float32);
                    } else {
                        types.insert(result.0, Type::Unknown);
                    }
                }
                Op::OpTypeVector {
                    result,
                    component_type,
                    component_count,
                } => {
                    if let Some(t) = types.get(&component_type.0) {
                        if let Type::Float32 = t {
                            match component_count {
                                2 => {
                                    types.insert(result.0, Type::Vec2);
                                }
                                3 => {
                                    types.insert(result.0, Type::Vec3);
                                }
                                4 => {
                                    types.insert(result.0, Type::Vec4);
                                }
                                _ => {
                                    types.insert(result.0, Type::Unknown);
                                }
                            }
                        } else {
                            types.insert(result.0, Type::Unknown);
                        }
                    } else {
                        return Err(Error::InvalidId);
                    }
                }
                Op::OpTypeMatrix {
                    result,
                    column_type,
                    column_count,
                } => {
                    let t = types
                        .get(&column_type.0)
                        .map(|column_type| match column_type {
                            Type::Vec3 if *column_count == 3 => Type::Mat3,
                            Type::Vec4 if *column_count == 4 => Type::Mat4,
                            _ => Type::Unknown,
                        })
                        .unwrap_or(Type::Unknown);
                    types.insert(result.0, t);
                }
                Op::OpTypeImage {
                    result,
                    sampled_type,
                    dim,
                    depth,
                    arrayed: _,
                    ms: _,
                    sampled,
                    format,
                    access: _,
                } => {
                    let t = if let Some(Type::Float32) = types.get(&sampled_type.0) {
                        if let Dim::D2 {} = dim {
                            Type::Image2D {
                                depth: *depth != 0,
                                sampled: *sampled != 0,
                                format: *format,
                            }
                        } else {
                            Type::Unknown
                        }
                    } else {
                        Type::Unknown
                    };
                    types.insert(result.0, t);
                }
                Op::OpTypeSampler { result } => {
                    types.insert(result.0, Type::Sampler);
                }
                Op::OpTypeSampledImage { result, image_type } => {
                    let t = if let Some(Type::Image2D { .. }) = types.get(&image_type.0) {
                        Type::SampledImage {
                            image_type_id: image_type.0,
                        }
                    } else {
                        Type::Unknown
                    };
                    types.insert(result.0, t);
                }
                Op::OpTypeArray {
                    result,
                    element_type,
                    length,
                } => {
                    if let Some(length) = constants.get(&length.0) {
                        types.insert(
                            result.0,
                            Type::Array {
                                element_type_id: element_type.0,
                                length: Some(*length),
                            },
                        );
                    } else {
                        return Err(Error::InvalidId);
                    }
                }
                Op::OpTypeRuntimeArray {
                    result,
                    element_type,
                } => {
                    types.insert(
                        result.0,
                        Type::Array {
                            element_type_id: element_type.0,
                            length: None,
                        },
                    );
                }
                Op::OpTypeStruct {
                    result,
                    element_types,
                } => {
                    types.insert(
                        result.0,
                        Type::Struct {
                            name: None,
                            elements: element_types
                                .iter()
                                .map(|e| StructMember {
                                    name: None,
                                    type_id: e.0,
                                    offset: None,
                                })
                                .collect(),
                        },
                    );
                }
                Op::OpTypePointer {
                    result,
                    storage_class,
                    pointed_type,
                } => {
                    types.insert(
                        result.0,
                        Type::Pointer {
                            storage_class: match storage_class {
                                ops::StorageClass::Unknown => StorageClass::Unknown,
                                ops::StorageClass::UniformConstant {}
                                | ops::StorageClass::Uniform {} => StorageClass::Uniform,
                                ops::StorageClass::PushConstant {} => StorageClass::PushConstant,
                            },
                            pointed_type_id: pointed_type.0,
                        },
                    );
                }
                Op::OpConstant {
                    result_type,
                    result,
                    value,
                } => {
                    if let Some(Type::UInt32) = types.get(&result_type.0) {
                        if value.len() == 1 {
                            constants.insert(result.0, value[0]);
                        }
                    }
                }
                Op::OpVariable {
                    result_type,
                    result,
                    storage_class: _,
                    initializer: _,
                } => {
                    vars.insert(
                        result.0,
                        Variable {
                            set: None,
                            binding: None,
                            type_id: result_type.0,
                            name: None,
                        },
                    );
                }
                _ => {}
            }
        }

        Ok(())
    }
}

/// Represents a type declared in a SPIRV module.
///
/// Types are declared in a hierarchy, with e.g. pointers relying on previously declared types as pointed-to types.
#[derive(Debug)]
pub enum Type {
    /// An unsupported type
    Unknown,
    /// The Void type
    Void,
    /// A boolean
    Bool,
    /// A signed 32-Bit integer
    Int32,
    /// An unsigned 32-Bit integer
    UInt32,
    /// A 32-Bit float
    Float32,
    /// A 2 component, 32-Bit vector (GLSL: vec2)
    Vec2,
    /// A 3 component, 32-Bit vector (GLSL: vec3)
    Vec3,
    /// A 4 component, 32-Bit vector (GLSL: vec4)
    Vec4,
    /// A 3x3, 32-Bit Matrix (GLSL: mat3)
    Mat3,
    /// A 4x4, 32-Bit Matrix (GLSL: mat4)
    Mat4,
    /// A 2D image
    Image2D {
        /// true if this image is a depth image
        depth: bool,
        /// true if this image can be sampled from
        sampled: bool,
        /// SPIRV code of the images format (should always be 0 in Vulkan)
        format: u32,
    },
    /// An opaque sampler object
    Sampler,
    /// A combined image and sampler (Vulkan: CombinedImageSampler descriptor)
    SampledImage {
        /// type id of the image contained in the SampledImage
        image_type_id: u32,
    },
    /// Either a static array with known length (`length` is [`Some`]) or dynamic array with unknown length (`length` is [`None`])
    Array {
        /// type id of the contained type
        element_type_id: u32,
        /// length of the array (if known)
        length: Option<u32>,
    },
    /// A struct containing other types
    Struct {
        name: Option<String>,
        /// members of the struct, in the order they appear in the SPIRV module (not necessarily ascending offsets)
        elements: Vec<StructMember>,
    },
    /// A pointer pointing to another type
    Pointer {
        /// The type of storage this pointer points to
        storage_class: StorageClass,
        /// The type id of the pointed-to type
        pointed_type_id: u32,
    },
}

/// Describes a single member of a [`Type::Struct`] type
#[derive(Debug)]
pub struct StructMember {
    pub name: Option<String>,
    pub type_id: u32,
    pub offset: Option<u32>,
}

/// Describes what type of storage a pointer points to
#[derive(Debug)]
pub enum StorageClass {
    Unknown,
    /// The pointer is a uniform variable (Uniform blocks)
    Uniform,
    /// The pointer is a uniform variable (Images, etc.)
    UniformConstant,
    /// The pointer is a push constant
    PushConstant,
}

/// Describes a variable declared in a SPIRV module
#[derive(Debug, Clone)]
pub struct Variable {
    /// Which DescriptorSet the variable is contained in (if known)
    pub set: Option<u32>,
    /// Which DescriptorSet binding the variable is contained in (if known)
    pub binding: Option<u32>,
    /// The type id of the variable's [`Type`]
    pub type_id: u32,
    /// The variables name (if known)
    pub name: Option<String>,
}
