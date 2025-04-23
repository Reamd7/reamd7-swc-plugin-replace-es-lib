use replace_es_lib::{replace_es_lib, Config};
use std::{fs::read_to_string, path::PathBuf};
use swc_core::{
    common::{chain, Mark},
    ecma::parser::{EsConfig, Syntax},
    ecma::transforms::{
        base::resolver,
        testing::{test_fixture, FixtureTestConfig},
    },
};
use testing::fixture;

fn syntax() -> Syntax {
    Syntax::Es(EsConfig {
        jsx: true,
        ..Default::default()
    })
}

#[fixture("tests/fixture/**/input.js")]
fn fixture(input: PathBuf) {
    let dir = input.parent().unwrap();
    let config = read_to_string(dir.join("config.json")).expect("failed to read config.json");
    println!("---- Config -----\n{}", config);
    let config: Config = serde_json::from_str(&config).unwrap();
    test_fixture(
        syntax(),
        &|_t| {
            chain!(
                resolver(Mark::new(), Mark::new(), false),
                replace_es_lib(config.clone()),
            )
        },
        &input,
        &dir.join("output.js"),
        Default::default(),
    );
}
