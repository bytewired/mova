use std::process;

pub fn exit_with_err_msg(message: &str) -> ! {
    println!("{message}");
    process::exit(1)
}

#[cfg(debug_assertions)]
pub fn print_debug(msg: &str) {
    println!("{}", msg)
}
