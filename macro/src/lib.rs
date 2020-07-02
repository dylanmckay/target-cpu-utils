extern crate proc_macro;

mod fetch_cpu;
mod macro_impl;

use proc_macro::*;

#[proc_macro]
pub fn cpu_name(input: proc_macro::TokenStream) -> TokenStream {
    ensure_empty_stream(input);

    match fetch_cpu::target_cpu().expect("failed to fetch target CPU name") {
        Some(cpu_name) => format!("\"{}\"", cpu_name).parse().unwrap(),
        None => {
            panic!("target CPU is not available");
        },
    }
}

#[proc_macro]
pub fn maybe_cpu_name(input: proc_macro::TokenStream) -> TokenStream {
    ensure_empty_stream(input);

    match fetch_cpu::target_cpu() {
        Ok(Some(cpu_name)) => format!("Some(\"{}\")", cpu_name).parse().unwrap(),
        _ => "None".parse().unwrap(),
    }
}

#[proc_macro_attribute]
pub fn cfg_target_cpu_eq(args: TokenStream, input: TokenStream) -> TokenStream {
    macro_impl::cfg_target_cpu(args, input, macro_impl::Condition::Equal)
}

#[proc_macro_attribute]
pub fn cfg_target_cpu_neq(args: TokenStream, input: TokenStream) -> TokenStream {
    macro_impl::cfg_target_cpu(args, input, macro_impl::Condition::NotEqual)
}


fn ensure_empty_stream(input: proc_macro::TokenStream) {
    if !input.is_empty() {
        panic!("no arguments expected to target CPU name macro");
    }

}
