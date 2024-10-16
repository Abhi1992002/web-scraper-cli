use clap::Parser;
use anyhow::{Context, Result};
use std::io::{self, Write};
use indicatif::ProgressBar;
use std::thread;
use std::time::Duration;
use log::{debug, error, info};

#[derive(Parser)] // It will parse the arguments you gave in commands into these fields
struct Cli{
    #[arg(short='p',long="pattern")]
    pattern : String,
    path : std::path::PathBuf , // Note: PathBuf is like a String but for file system paths that work cross-platform.
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()>{
    // we will going to use the std::env::args() - It gives you the iterators of the given arguments
    // firsst entry is 0, this is the program name of yours
    // we executed something like grrs foobar test.txt

    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern,
    //     path : std::path::PathBuf::from(path)
    // };

    // println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
    env_logger::init(); // if we give RUST_LOG=error, it will only print Error, if info - then print Error and info and if debug, then print all 3
    // we are using this adapter to see a particular type of log, by passing the enviromnemnt variable RUST_LOG=

    let args = Cli::parse();

    // // Opening the file to read??
    // // This is not the right way to read the file, you should instead use BufRead
    // let result = std::fs::read_to_string(&args.path);

    // let content = match result{
    //     Ok(content) => { content }
    //     // Err(error) => { println!("Oh noes: {}", error); } // It would create a panic
    //     Err(error) => { return Err(error.into()); } // this will not panic the thread, It return the error
    // };


    // Shortcut - let result = std::fs::read_to_string(&args.path).unwrap(); -> this is shortcut for match with panic
    // Shortcut - let result = std::fs::read_to_string(&args.path)?; -> this is shortcut for match with returing error
    // read_to_string return type of error as std::io::Error, but we are returung Box<dyn std::error::Error> because ? handling return it
    
    // If you uses ?, then the error is something like Error: Os { code: 2, kind: NotFound, message: "No such file or directory" }

    // It is very less informative 

    // let path = "test.txt";
    // let content = std::fs::read_to_string(path)
    //     .map_err(|err| CustomError(format!("Error reading `{}`: {}", path, err)))?;
    // println!("file content: {}", content);

    // WE you build a custom error then you should have to return Result<(),CustomError>

    // using this map_err is very common, but we do not want to store err, so in real life we uses library called anyhow
    let content = std::fs::read_to_string(&args.path).with_context(|| format!("could not read file `{:?}`", args.path.display()))?;
    // Just return the Result imported from the Result
    
    grrs::find_matches(&content,&args.pattern,&mut std::io::stdout()); // stdout implement write trait and with the help of this we can write daa on terminal

    // we got below error
    // Error: could not read file `test.txt`
    // Caused by:
    // No such file or directory (os error 2)

    // I am using this command - cargo run -- -p=main some-main.rs
    
    // Output 
    // We do not want to flush() on terminal everytime, we could print ihem into a block
    // we store the print into buffer 

    let stdout = io::stdout();
    let mut handler = io::BufWriter::new(stdout); 
    writeln!(handler,"foo: {}",42);

    // prevents the system from locking and unlocking stdout over and over again.
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    writeln!(handle, "foo: {}", 42);

    // we can add progress bar using indicatif
    // let pb = ProgressBar::new(1024);


    // for _ in 0..1024{
    //     thread::sleep(Duration::from_millis(5));
    //     pb.inc(1);
    // }

    // pb.finish_with_message("done");

    info!("Print Somrthing");
    error!("error");
    debug!("debug");

   

    Ok(())
     
}


// I want to write the test for the find_matches
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    grrs::find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result); // we have write the content on result
    // find_matches function will goiing to print the output on the terminal,
    // so how could we compare it
    // we need to make it testable, you know to write something, stdout function uses Write trait
    // Why Use std::io::Write Instead of std::fmt::Write?
    // stdout expects bytes, io::Write work with bytes but fmt::Write work with string
    // but result vextor captures the data in bytes, so i has converted the string in bytes
    assert_eq!(result, b"lorem ipsum\n")
}        


fn answer() -> i32{
    42
}

#[test]
fn check_answer_validity(){
    assert_eq!(answer(),42)
}