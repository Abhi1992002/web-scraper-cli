use clap::{Parser};

#[derive(Parser)]
#[derive(Debug)]
#[command(name = "Web Crawler", version = "1.0", author = "Abhimanyu", about = "A multithreaded web crawler")]
pub struct CliIntialArgs{
    #[arg(short = 'u', long = "url", value_name = "URL", help = "The starting URL to crawl", required = true)]
    url : String,

    #[arg(short = 'd', long = "depth", value_name = "DEPTH", default_value_t = String::from("3"), help = "The maximum depth to crawl")]
    depth : String,

    #[arg(short = 't', long = "threads", value_name = "THREADS", default_value_t = String::from("4"), help = "Number of threads to use")]
    threads: String,

    #[arg(short = 'o', long = "output", value_name = "FILE", help = "Output file for crawled URLs")]
    output: String
}

pub fn new() -> CliIntialArgs{
   let intitial_args = CliIntialArgs::parse();
   
 
   return intitial_args;
}