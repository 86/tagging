use anyhow::Result;
use structopt::StructOpt;

use super::tag;

#[derive(StructOpt, Debug)]
#[structopt(name = "tagging")]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Specify tag prefix
    #[structopt(short, long)]
    pub prefix: String,
}

pub struct Cli {
    pub opt: Opt,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            opt: Opt::from_args(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let input = self.prompt("> 🤖 Please input a tag manually:")?;
        let tag = tag::Tag::new(&input, self.opt.prefix.clone())?;
        println!("✨ The new tag: {}", tag.to_string());
        Ok(())
    }

    fn prompt(&self, description: &str) -> std::io::Result<String> {
        println!("{}", description);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.pop();
        Ok(input)
    }
}
