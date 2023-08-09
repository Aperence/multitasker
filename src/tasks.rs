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

#[cfg(test)]
mod tests{

    use crate::tasks::run_task;
    use std::str;

    #[test]
    fn test_run_task(){
        let out = run_task("cat src/tasks.rs | wc -l");

        assert_eq!(out.status.code().unwrap(), 0);
        assert_eq!(str::from_utf8(&out.stdout).unwrap().trim(), "28");
        assert_eq!(str::from_utf8(&out.stderr).unwrap().trim(), "");
    }
}