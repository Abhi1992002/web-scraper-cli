mod cli;
fn main() {
    // starting my cli
    let initial_args  = cli::new();
    println!("{:?}",initial_args);
}
