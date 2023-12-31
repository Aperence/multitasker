use std::{str::FromStr, fs, time::Instant};

use clap::Parser;
use tokio::sync::mpsc;

mod tasks;
mod multitasker;

/// Program used to run multiples tasks in parallel
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file where the commands will be read (one command per line)
    input: String,

    /// Name of the output file where the results will be saved
    #[arg(short, long, default_value_t = String::from_str("results.txt").unwrap())]
    output: String,


    /// Separator between each field (Cmd, timed, status code, stdout, stderr) of the task results
    #[arg(short, long, default_value_t=String::from_str(",").unwrap())]
    sep: String,

    /// Separator between each task
    #[arg(long, default_value_t=String::from_str("\n").unwrap())]
    task_sep: String,

    /// Verbosity of the program
    #[arg(short, long, action)]
    verbose: bool
}


#[tokio::main]
async fn main() {
    let args = Args::parse();
    let (tx, rx) = mpsc::channel(1000);

    let writer = multitasker::Writer::new(rx, args.output, args.sep, args.task_sep);

    let writer_handle = writer.run();

    for line in fs::read_to_string(args.input).expect("Failed to open input file").lines(){
        let t = tx.clone();
        let l = String::from_str(line).unwrap();
        tokio::task::spawn(async move{
            let i = Instant::now();
            let out = tasks::run_task(&l);
            let elapsed = i.elapsed();
            if args.verbose{
                println!("Done task \"{}\"", &l);
            }
            t.send(multitasker::TimedOutput{o : out, d : elapsed, s:l}).await.unwrap();
        });
    }

    drop(tx);

    writer_handle.await;  // wait for writer to end
}
