# SPIRV-Layout

[![MIT License](https://img.shields.io/badge/License-MIT-blue?style=for-the-badge)](https://choosealicense.com/licenses/mit/) 
[![Continuous integration](https://img.shields.io/github/actions/workflow/status/rob2309/spirv-layout/ci.yaml?style=for-the-badge)](https://github.com/rob2309/spirv-layout/actions) 
[![Crates.io](https://img.shields.io/crates/v/spirv-layout?style=for-the-badge)](https://crates.io/crates/spirv-layout)
[![docs.rs](https://img.shields.io/docsrs/spirv-layout?style=for-the-badge)](https://docs.rs/spirv-layout)

This library parses SPIRV binaries and retrieves reflection info.
It is most useful for deriving a Vulkan `DescriptorSetLayout` from a shader module, as well as finding offsets and names of individual fields in the Uniform Buffers of a shader.

This crate is used by the [vulkan-engine](https://github.com/michidk/vulkan-engine) project.

## Usage

```rust
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
```

For an actual usage example, see [`examples/reflect-shader`](examples/reflect-shader/main.rs)
