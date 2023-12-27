use std::env;
use std::io;

mod modules {
    pub mod cleaner;
}

use modules::cleaner::run_module;

fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;
    let cwd_str = &cwd.into_os_string().into_string().unwrap();

    run_module(&cwd_str).expect("An error ocured");

    Ok(())
}