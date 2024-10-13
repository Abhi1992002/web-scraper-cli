use std::io::{self, Write};
use colored::*;
use clap::{Parser,Subcommand};
use reqwest::Url;
use crate::crawler::fetcher::fetch_url;


#[derive(Parser)]
#[derive(Debug)]
#[command(name = "Web Crawler", version = "1.0", author = "Abhimanyu", about = "A multithreaded web crawler")]
pub struct CliIntialArgs{
    #[command(subcommand)]
    command : Commands,
    
}

#[derive(Subcommand)]
#[derive(Debug)]
enum Commands{
   Scrape {
    #[arg(short = 'u', long = "url", value_name = "URL", help = "The starting URL to crawl")]
    url : Option<String>,

    #[arg(short = 'd', long = "depth", value_name = "DEPTH", default_value_t = String::from("3"), help = "The maximum depth to crawl")]
    depth : String,

    #[arg(short = 't', long = "threads", value_name = "THREADS", default_value_t = String::from("4"), help = "Number of threads to use")]
    threads: String,

    #[arg(short = 'o', long = "output", value_name = "FILE", help = "Output file for crawled URLs")]
    output: Option<String>,
   }
}




pub async  fn new() -> CliIntialArgs{
   let mut intitial_args = CliIntialArgs::parse();

   // destructuring the commands
   let Commands::Scrape { ref mut url, ref depth, ref threads, ref mut output } = intitial_args.command;
 
   if url.is_none(){
    *url = Some(ask_for_url())
   }

   if output.is_none(){
    *output = Some(ask_for_output())
   }

   let url_copy = url.clone().unwrap();
   let output_copy = output.clone().unwrap();

   println!("\nScraping the url: {}", url_copy.green());
   println!("Number of threads used: {}", threads.green());
   println!("Maximum depth: {}", depth.green());
   println!("Output will be stored in: {}", output_copy.green());

   println!("\nStarting the Scraping...");

   if let (Some(url),Some(output)) = (url.as_ref(),output.as_ref()) {
    
    let html_content = fetch_url(url).await;

    if let Ok(val) = html_content {
        println!("HTML Content : {}",val);
    }
 

   }
 
   return intitial_args;
}


fn ask_for_url() -> String{
   print!("\nPlease Enter your url : ");
   io::stdout().flush().unwrap(); // print will store in buffer, so i am displaying it (You do not have to write it in println)

   let mut url1 = String::from("");
   io::stdin().read_line(&mut url1).expect("Failed to return input");
   url1.trim().to_string();
   url1
}

fn ask_for_output() -> String{
    print!("\nPlease enter where you want to store input : ");
    io::stdout().flush().unwrap();
    let mut output_path = String::from("");
    io::stdin().read_line(&mut output_path).expect("Getting wrror in output_path");
    output_path.trim().to_string();
    output_path
}
