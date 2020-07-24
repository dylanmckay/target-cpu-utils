//! Provides a method for querying the CPU name from the current target specification JSON passed
//! to Rust.
//!
//! See also the crate [target-cpu-macro](https://crates.io/crates/target-cpu-macro) for
//! conditional compilation support.

use std::path::Path;

/// Returns the name of the target CPU.
///
/// Looks at the name of the current target specification JSON passed to `rustc` and
/// derives that target from it.
///
/// If there is an error during the lookup, `Err(message)` is returned. If the target
/// specification could be found, but it was empty, then `Ok(None)` is returned.
pub fn target_cpu() -> Result<Option<String>, String> {
    // N.B. This environment variable can be used for testing. I didn't want to prefix
    // with 'RUST_', this will work sufficiently well as a scare tactic against usage.
    if let Ok(force_override_cpu) = std::env::var("_DYLAN_RUST_TARGET_CPU_FORCE_OVERRIDE").map(|s| s.trim().to_owned()) {
        if !force_override_cpu.is_empty() {
            eprintln!("warning: force overriding CPU detection in crate '{}' to CPU '{}'",
                env!("CARGO_PKG_NAME"), force_override_cpu);

            return Ok(Some(force_override_cpu));
        }
    }

    let target = if let Ok(target) = std::env::var("TARGET") {
        target
    } else {
        return Err("cannot retrieve CPU name, please, pass --target flag to Cargo, e. g. \"--target atmega88pa.json\"".to_owned());
    };

    let target_json_relative_path = Path::new(&format!("{}.json", target)).to_owned();

    let cpu_name = match parse_target_cpu_from_target_json(&target_json_relative_path) {
        Some(target_cpu) => target_cpu,
        None => {
            // TARGET environment variable should contain the value that was specified
            // by --target X(C)argo option. Normally it's the name of .json file
            // containing custom target specification, e. g. atmega88pa.json
            // So in order to work, the name of *.json file should be the same
            // as the name of your MCU
            eprintln!("[warning]: assuming a target CPU name of '{}' from the file name of the target spec JSON file", target);

            target
        },
    };

    Ok(Some(cpu_name))
}


fn parse_target_cpu_from_target_json(possible_json_path: &Path)
    -> Option<String> {
    let json_content = match std::fs::read(possible_json_path) {
        Ok(content) => String::from_utf8(content).unwrap(),
        Err(..) => {
            eprintln!("[warning]: cannot find target JSON file '{}' - due to limitations it needs to be in the crate root - is it?", possible_json_path.display());
            return None;
        },
    };

    let parsed = json::parse(&json_content).expect(&format!("failed to parse target JSON at '{}'", possible_json_path.display()));

    match parsed {
        json::JsonValue::Object(dict) => {
            match dict.get("cpu") {
                Some(target_cpu) => Some(target_cpu.as_str().expect("target CPU in JSON is not a string").to_owned()),
                None => panic!("target specification file '{}' does not define a target CPU", possible_json_path.display()),
            }
        },
        _ => None,
    }
}
