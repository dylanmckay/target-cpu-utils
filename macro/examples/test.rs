#[macro_use] extern crate target_cpu_macro;

const FOO: Option<&'static str> = target_cpu_macro::maybe_cpu_name!();

#[cfg_target_cpu_eq(atmega328p)]
const VALUE: u8 = 1;

#[cfg_target_cpu_neq(atmega328p)]
const VALUE: u8 = 2;

fn main() {
    println!("CPU NAME: {:?}", FOO);
    println!("VALUE: {:?}", VALUE);
}

