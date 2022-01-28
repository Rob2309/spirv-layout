use std::ffi::CStr;

use crate::{Error, SpirvResult};

/// Used to more easily declare the relevant SPIRV opcodes
macro_rules! ops {
    (
        $($opcode:literal = $op_name:ident( $($arg_name:ident: $arg_type:ty),* )),* $(,)?
    ) => {
        #[derive(Debug)]
        #[allow(dead_code)]
        #[allow(clippy::enum_variant_names)]
        pub(crate) enum Op {
            Unknown,

            $($op_name{ $($arg_name: $arg_type),* }),*
        }

        impl Op {
            pub(crate) fn decode(stream: &mut &[u32]) -> SpirvResult<Self> {
                if stream.len() < 1 {
                    return Err(Error::Other("unexpected end of stream".to_owned()));
                }

                let opcode_and_length = stream[0];
                let opcode = (opcode_and_length & 0xFFFF) as u16;
                let length = ((opcode_and_length >> 16) & 0xFFFF) as u16;

                if length == 0 || length as usize > stream.len() {
                    return Err(Error::InvalidOp);
                }

                let mut op_stream = &stream[1..length as usize];

                let op = match opcode {
                    $($opcode => {
                        $(let $arg_name = <$arg_type>::decode_arg(&mut op_stream)?;)*
                        Self::$op_name{ $($arg_name),* }
                    }),*

                    _ => Op::Unknown,
                };

                *stream = &stream[length as usize..];
                Ok(op)
            }
        }
    };
}

ops!(
    5 = OpName(target: Id, name: String),
    6 = OpMemberName(target: Id, member_index: u32, name: String),
    15 = OpEntryPoint(
        execution_model: ExecutionModel,
        func: Id,
        name: String,
        interface: Vec<Id>
    ),
    71 = OpDecorate(target: Id, decoration: Decoration),
    72 = OpMemberDecorate(target: Id, member_index: u32, decoration: Decoration),
    19 = OpTypeVoid(result: Id),
    20 = OpTypeBool(result: Id),
    21 = OpTypeInt(result: Id, width: u32, signed: u32),
    22 = OpTypeFloat(result: Id, width: u32),
    23 = OpTypeVector(result: Id, component_type: Id, component_count: u32),
    24 = OpTypeMatrix(result: Id, column_type: Id, column_count: u32),
    25 = OpTypeImage(
        result: Id,
        sampled_type: Id,
        dim: Dim,
        depth: u32,
        arrayed: u32,
        ms: u32,
        sampled: u32,
        format: u32,
        access: Option<u32>
    ),
    26 = OpTypeSampler(result: Id),
    27 = OpTypeSampledImage(result: Id, image_type: Id),
    28 = OpTypeArray(result: Id, element_type: Id, length: Id),
    29 = OpTypeRuntimeArray(result: Id, element_type: Id),
    30 = OpTypeStruct(result: Id, element_types: Vec<Id>),
    32 = OpTypePointer(result: Id, storage_class: StorageClass, pointed_type: Id),
    43 = OpConstant(result_type: Id, result: Id, value: Vec<u32>),
    59 = OpVariable(
        result_type: Id,
        result: Id,
        storage_class: StorageClass,
        initializer: Option<Id>
    ),
);

trait DecodeArg {
    fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self>
    where
        Self: Sized;
}

impl DecodeArg for u32 {
    fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self>
    where
        Self: Sized,
    {
        if stream.is_empty() {
            Err(Error::InvalidOp)
        } else {
            let arg = stream[0];
            *stream = &stream[1..];
            Ok(arg)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Id(pub(crate) u32);

impl DecodeArg for Id {
    fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self>
    where
        Self: Sized,
    {
        Ok(Id(u32::decode_arg(stream)?))
    }
}

impl DecodeArg for String {
    fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self>
    where
        Self: Sized,
    {
        let mut num_words = 0;

        if stream.iter().any(|e| {
            num_words += 1;
            e.to_le_bytes().iter().any(|b| *b == 0)
        }) {
            let arg = unsafe { CStr::from_ptr(stream.as_ptr().cast::<i8>()) }
                .to_str()?
                .to_owned();
            *stream = &stream[num_words..];
            Ok(arg)
        } else {
            Err(Error::InvalidOp)
        }
    }
}

impl<T: DecodeArg> DecodeArg for Option<T> {
    fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self>
    where
        Self: Sized,
    {
        if stream.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::decode_arg(stream)?))
        }
    }
}

impl<T: DecodeArg> DecodeArg for Vec<T> {
    fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self>
    where
        Self: Sized,
    {
        let mut res = Vec::new();

        while !stream.is_empty() {
            let arg = T::decode_arg(stream)?;
            res.push(arg);
        }

        Ok(res)
    }
}

macro_rules! enums {
    ($(
        $enum_name:ident {
            $($variant_code:literal = $variant_name:ident ( $($arg_name:ident: $arg_type:ty),* $(,)? )),* $(,)?
        }
    ),* $(,)?) => {
        $(
            #[derive(Debug)]
            pub(crate) enum $enum_name {
                Unknown,
                $($variant_name { $($arg_name: $arg_type),* }),*
            }

            impl DecodeArg for $enum_name {
                fn decode_arg(stream: &mut &[u32]) -> SpirvResult<Self> where Self: Sized {
                    if stream.len() < 1 {
                        return Err(Error::InvalidOp);
                    }

                    let code = stream[0];
                    *stream = &stream[1..];

                    match code {
                        $(
                            $variant_code => {
                                $(let $arg_name = <$arg_type>::decode_arg(stream)?;)*
                                Ok(Self::$variant_name {
                                    $($arg_name),*
                                })
                            }
                        ),*

                        _ => Ok(Self::Unknown),
                    }
                }
            }
        )*
    };
}

enums!(
    Decoration {
        4 = RowMajor(),
        5 = ColMajor(),
        7 = MatrixStride(stride: u32),
        30 = Location(loc: u32),
        33 = Binding(binding: u32),
        34 = DescriptorSet(set: u32),
        35 = Offset(offset: u32),
    },

    Dim {
        0 = D1(),
        1 = D2(),
        2 = D3(),
        3 = Cube(),
        4 = Rect(),
        5 = Buffer(),
        6 = SubpassData(),
    },

    StorageClass {
        0 = UniformConstant(),
        1 = Input(),
        2 = Uniform(),
        3 = Output(),
        9 = PushConstant(),
    },

    ExecutionModel {
        0 = Vertex(),
        4 = Fragment(),
    },
);
