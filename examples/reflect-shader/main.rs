use core::slice;

use spirv_layout::{LocationVariable, Module, PushConstantVariable, Type, UniformVariable};

const PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/reflect-shader/test2.spv"
);

fn main() {
    {
        let bytes = std::fs::read(PATH).unwrap();
        let words = unsafe { slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let module = Module::from_words(words).unwrap();

        for ep in module.get_entry_points() {
            println!("ENTRYPOINT {} {:?}", ep.name, ep.execution_model);

            println!("=== INPUTS ===");
            for var in &ep.inputs {
                print_location_var(&module, var);
            }

            println!("=== OUTPUTS ===");
            for var in &ep.outputs {
                print_location_var(&module, var);
            }

            println!("=== UNIFORMS ===");
            for var in &ep.uniforms {
                print_uniform_var(&module, var);
            }

            println!("=== PUSH CONSTANTS ===");
            for var in &ep.push_constants {
                print_pc_var(&module, var);
            }
        }
    }
}

fn print_uniform_var(module: &Module, var: &UniformVariable) {
    print!("layout (set={}, binding={}) ", var.set, var.binding);

    print_type(module, module.get_type(var.type_id).unwrap());

    println!(
        "{}; // size={}",
        if let Some(name) = &var.name {
            name
        } else {
            "<no-name>"
        },
        module.get_var_size(var).unwrap()
    );
}

fn print_pc_var(module: &Module, var: &PushConstantVariable) {
    print_type(module, module.get_type(var.type_id).unwrap());

    println!(
        "{}; // size={}",
        if let Some(name) = &var.name {
            name
        } else {
            "<no-name>"
        },
        module.get_var_size(var).unwrap()
    );
}

fn print_location_var(module: &Module, var: &LocationVariable) {
    print!("layout (location={}) ", var.location);

    print_type(module, module.get_type(var.type_id).unwrap());

    println!(
        "{}; // size={}",
        if let Some(name) = &var.name {
            name
        } else {
            "<no-name>"
        },
        module.get_var_size(var).unwrap()
    );
}

fn print_type(module: &Module, ty: &Type) {
    match ty {
        Type::Void => print!("void "),
        Type::Bool => print!("bool "),
        Type::Int32 => print!("int "),
        Type::UInt32 => print!("uint "),
        Type::Float32 => print!("float "),
        Type::Vec2 => print!("vec2 "),
        Type::Vec3 => print!("vec3 "),
        Type::Vec4 => print!("vec4 "),
        Type::Mat3 => print!("mat3 "),
        Type::Mat4 => print!("mat4 "),
        Type::Image2D { .. } => print!("image2D "),
        Type::Sampler => print!("sampler "),
        Type::SampledImage { .. } => print!("sampler2D "),
        Type::Array {
            element_type_id,
            length,
        } => {
            print_type(module, module.get_type(*element_type_id).unwrap());
            print!("[{}] ", length.unwrap_or(0));
        }
        Type::Struct { name, elements } => {
            println!(
                "struct {} {{",
                name.as_ref().map(|n| n.as_str()).unwrap_or("<no-name>")
            );

            for elem in elements {
                print!("    layout(");
                if let Some(offset) = elem.offset {
                    print!("offset={}", offset);
                }
                if let Some(Type::Mat3 | Type::Mat4) = module.get_type(elem.type_id) {
                    print!(
                        ", {}, stride={}",
                        if elem.row_major {
                            "row_major"
                        } else {
                            "col_major"
                        },
                        elem.stride
                    );
                }
                print!(") ");

                print_type(module, module.get_type(elem.type_id).unwrap());

                println!(
                    "{}; // size={}",
                    if let Some(name) = &elem.name {
                        name
                    } else {
                        "<no-name>"
                    },
                    module.get_member_size(elem).unwrap()
                );
            }

            print!("}} ");
        }
        Type::Pointer {
            storage_class: _,
            pointed_type_id,
        } => {
            print_type(module, module.get_type(*pointed_type_id).unwrap());
            print!("* ");
        }
        _ => print!("<unknown> "),
    }
}
