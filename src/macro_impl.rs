use crate::fetch_cpu;
use proc_macro::*;

#[derive(Copy, Clone, Debug)]
pub enum Condition {
    Equal,
    NotEqual,
}

pub fn cfg_target_cpu(
    args: TokenStream,
    input: TokenStream,
    condition: Condition,
) -> TokenStream {
    let predicated_cpu_name = args.to_string();
    let actual_cpu_name = fetch_cpu::target_cpu().expect("could not find CPU name for conditional compilation").expect("could not find CPU name for conditional compilation");

    let should_preserve = match condition {
        Condition::Equal => predicated_cpu_name == actual_cpu_name,
        Condition::NotEqual => predicated_cpu_name != actual_cpu_name,
    };

    if should_preserve {
        input
    } else {
        "".parse().unwrap()
    }
}
