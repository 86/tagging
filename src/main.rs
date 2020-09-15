use tagging::cli;

fn main() {
    let cli = cli::Cli::new();
    if cli.opt.debug {
        println!("{:?}", cli.opt);
    }
    match  cli.run() {
        Ok(_) => println!("✅ Done."),
        Err(error) => eprintln!("🛑 Error: {}", error),
    }
}
