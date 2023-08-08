use std::process::{Command, Output};

/**
 * Run a cmd and return the result of this cmd
 */
pub fn run_task(cmd : &str) -> Output{
    Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .output()
                .expect(format!("Failed to run {}", cmd).as_str())
}