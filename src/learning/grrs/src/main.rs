use clap::Parser;

#[derive(Parser)] // It will parse the arguments you gave in commands into these fields
struct Cli{
    #[arg(short='p',long="pattern")]
    pattern : String,
    path : std::path::PathBuf , // Note: PathBuf is like a String but for file system paths that work cross-platform.
}

fn main() {
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

    // Opening the file to read??
    // This is not the right way to read the file, you should instead use BufRead
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
    // I am using this command - cargo run -- -p=main some-main.rs


     
}
