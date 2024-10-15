use clap::Parser;
use anyhow::{Context, Result};

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

    // we got below error
    // Error: could not read file `test.txt`
    // Caused by:
    // No such file or directory (os error 2)

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
    // I am using this command - cargo run -- -p=main some-main.rs

    Ok(())
     
}
