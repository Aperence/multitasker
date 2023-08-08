use tokio::sync::mpsc::Receiver;
use std::time::Duration;
use std::{process::Output, fs::File, io::Write};
use std::str;

pub struct TimedOutput{
    pub d : Duration, 
    pub o : Output
}

pub struct Writer{
    rx : Receiver<TimedOutput>,
    output : File,
    sep : String,
    task_sep : String,
    first : bool
}

impl Writer{
    pub fn new(rx : Receiver<TimedOutput>, output : String, sep : String, task_sep : String,) -> Writer{
        let f = File::create(output).expect("Failed to create output file");
        Writer{rx, output:f, sep, task_sep, first:true}
    }

    pub async fn run(mut self){
        loop{
            let out = self.rx.recv().await;
            match  out {
                None => break,  // all transmiter dropped, we have done all the tasks
                Some(out) => {
                    if !self.first{
                        self.output.write_all(self.task_sep.as_bytes()).expect("Failed to write file");
                    }else{
                        self.first = false;
                    }
                    self.output.write_all(out.d.as_millis().to_string().as_bytes()).expect("Failed to write file");
                    self.output.write_all(self.sep.as_bytes()).expect("Failed to write file");

                    self.output.write_all(out.o.status.code().unwrap().to_string().as_bytes()).expect("Failed to write file");
                    self.output.write_all(self.sep.as_bytes()).expect("Failed to write file");

                    self.output.write_all(str::from_utf8(&out.o.stdout).unwrap().trim().as_bytes()).expect("Failed to write file");
                    self.output.write_all(self.sep.as_bytes()).expect("Failed to write file");
                    
                    self.output.write_all(str::from_utf8(&out.o.stderr).unwrap().trim().as_bytes()).expect("Failed to write file");
                }
            }
        }
    }
}