use std::fmt::Display;
use std::process::exit;

mod commands;
mod config;
mod init;
mod web;

fn main() {
    crate::commands::run();
}

fn exit_with_msg(code: i32, err: &Display) -> ! {
    println!("{}", err);
    exit(code)
}
