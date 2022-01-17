use core::slice;

use spirv_layout::{Module, Type, Variable};

const PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/reflect-shader/test2.spv"
);

fn main() {
    {
        let bytes = std::fs::read(PATH).unwrap();
        let words = unsafe { slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) };
        let module = Module::from_words(words).unwrap();

        println!("=== UNIFORMS ===");
        for var in module.get_uniforms() {
            print_var(&module, var);
        }

        println!("=== PUSH CONSTANTS ===");
        for var in module.get_push_constants() {
            print_var(&module, var);
        }
    }
}

fn print_var(module: &Module, var: &Variable) {
    if let Some(set) = var.set {
        if let Some(binding) = var.binding {
            print!("layout (set={}, binding={}) ", set, binding);
        }
    }

    print_type(module, module.get_type(var.type_id).unwrap());

    println!(
        "{};",
        if let Some(name) = &var.name {
            name
        } else {
            "<no-name>"
        }
    );
}

fn print_type(module: &Module, ty: &Type) {
    match ty {
        Type::Unknown => print!("<unknown> "),
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
        Type::Struct { elements } => {
            println!("struct {{");

            for elem in elements {
                print!("    ");
                if let Some(offset) = elem.offset {
                    print!("layout(offset={}) ", offset);
                }

                print_type(module, module.get_type(elem.type_id).unwrap());

                println!(
                    "{};",
                    if let Some(name) = &elem.name {
                        name
                    } else {
                        "<no-name>"
                    }
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
    }
}
