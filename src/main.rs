use anyhow::Result;
use tagging::cli;

fn main() -> Result<()> {
    let cli = cli::Cli::new();
    if cli.opt.debug {
        println!("{:?}", cli.opt);
    }
    cli.run()?;
    println!("✅ Done.");
    Ok(())
}
