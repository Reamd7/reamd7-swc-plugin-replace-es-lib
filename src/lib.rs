use swc_core::{
    ecma::ast::{Pass, Program},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
pub fn replace_es_lib(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for replace_es_lib"),
    )
    .expect("invalid packages");

    replace_es_lib::replace_es_lib(config).process(&mut program);
    program
    // program.fold_with(tranformer)
}
