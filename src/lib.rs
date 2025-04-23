use replace_es_lib::Config;
use swc_core::{
    ecma::ast::{Pass, Program},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
pub fn replace_es_lib(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for replace_es_lib"),
    )
    .expect("invalid packages");

    if config.target_module.len() == 0 {
        return program;
    }

    if config.direction == "es2lib" || config.direction == "lib2es" {
        replace_es_lib::replace_es_lib(config).process(&mut program);
    } else {
        panic!("invalid direction");
    }

    program
}
