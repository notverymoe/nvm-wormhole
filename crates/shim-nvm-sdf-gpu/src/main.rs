use spirv_builder::{SpirvBuilder, ModuleResult, SpirvBuilderError, MetadataPrintout, Capability};

fn main() -> Result<(), SpirvBuilderError> {
    std::fs::create_dir_all("./output/").unwrap();
    for folder in std::fs::read_dir("./shaders/").unwrap().map(|v| v.unwrap()).filter(|v| v.file_type().unwrap().is_dir()) {
        let name = folder.file_name();
        let name = name.to_str().unwrap();
        compile_shader(name)?;
        println!("Output: {}.spv", name)
    }
    Ok(())
}


fn compile_shader(name: &str) -> Result<(), SpirvBuilderError> {
    let result = SpirvBuilder::new(
            format!("{}/shaders/{}", env!("CARGO_MANIFEST_DIR"), name), 
            "spirv-unknown-spv1.3"
        )
        .multimodule(true)
        .capability(Capability::Int8)
        .capability(Capability::Int16)
        .capability(Capability::Int64)
        .print_metadata(MetadataPrintout::DependencyOnly)
        .build()?;

    if let ModuleResult::SingleModule(result) = result.module {
        std::fs::copy(result, format!("output/{}.spv", name)).unwrap();
    }

    Ok(())
}