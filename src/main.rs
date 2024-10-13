mod cli;
mod crawler;

#[tokio::main]
async fn main() {
    // starting my cli
    // let initial_args  = cli::new().await;
    // println!("{:?}",initial_args);
    cli::new().await;
}
