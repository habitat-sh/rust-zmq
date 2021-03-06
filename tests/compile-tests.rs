#![cfg(feature = "unstable-testing")]

extern crate compiletest_rs as compiletest;

use std::env::var;
use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    let cfg_mode = mode.parse().expect("Invalid mode");

    config.target_rustcflags = Some("-L target/debug/ -L target/debug/deps/".to_owned());
    if let Ok(name) = var::<&str>("TESTNAME") {
        let s: String = name.to_owned();
        config.filter = Some(s)
    }
    config.mode = cfg_mode;
    config.src_base = PathBuf::from(format!("tests/{}", mode));

    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("compile-fail");
}
