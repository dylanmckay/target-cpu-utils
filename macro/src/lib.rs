//! A crate used to query the current target CPU for a crate.
//!
//! Provides macros for fetching the target CPU name, and custom
//! attributes that can be used to conditionally enable code
//! based on that CPU.

#![feature(proc_macro_hygiene)]

extern crate proc_macro;

use target_cpu_fetch as fetch_cpu;
mod macro_impl;

use proc_macro::*;

/// Expands into a `&'static str` string literal containing the current CPU name.
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

/// Expands into a `Option<&'static str>` containing the current CPU name.
#[proc_macro]
pub fn maybe_cpu_name(input: proc_macro::TokenStream) -> TokenStream {
    ensure_empty_stream(input);

    match fetch_cpu::target_cpu() {
        Ok(Some(cpu_name)) => format!("Some(\"{}\")", cpu_name).parse().unwrap(),
        _ => "None".parse().unwrap(),
    }
}

/// Conditionally enables a piece of code if the target CPU has the specified name.
#[proc_macro_attribute]
pub fn cfg_target_cpu_eq(args: TokenStream, input: TokenStream) -> TokenStream {
    macro_impl::cfg_target_cpu(args, input, macro_impl::Condition::Equal)
}

/// Conditionally enables a piece of code if the target CPU does not have the specified name.
#[proc_macro_attribute]
pub fn cfg_target_cpu_neq(args: TokenStream, input: TokenStream) -> TokenStream {
    macro_impl::cfg_target_cpu(args, input, macro_impl::Condition::NotEqual)
}

fn ensure_empty_stream(input: proc_macro::TokenStream) {
    if !input.is_empty() {
        panic!("no arguments expected to target CPU name macro");
    }

}
