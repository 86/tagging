use anyhow::Result;
use structopt::StructOpt;

use super::git::{GitRepo, GitRepoIO};
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
    git: GitRepo,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            opt: Opt::from_args(),
            git: GitRepo::new(),
        }
    }

    pub fn run(&self) -> Result<()> {
        let mut tags = self.git.get_tags(&self.opt.prefix)?;
        let mut new_tag: Option<tag::Tag> = Option::None;
        if tags.len() == 0 {
            let input = self.prompt("> 🤖 Please input a tag manually:")?;
            let tag = tag::Tag::new(&input, self.opt.prefix.clone(), Option::None)?;
            new_tag = Option::Some(tag);
        } else {
            if tags.len() > 3 {
                tags.truncate(3);
            }
            let latest_tags: String = tags
                .iter()
                .map(|tag| tag.to_string())
                .collect::<Vec<String>>()
                .join("\n");
            println!("Latest tags:\n{}", latest_tags);
            let mut position: Option<tag::Position> = Option::None;
            while position.is_none() {
                let input = self.prompt("\n🤖 Which position do you want to increment?\nmajor (M), minor (m), patch (p):")?;
                match input.as_str() {
                    "M" => {
                        position = Option::Some(tag::Position::Major);
                    }
                    "m" => {
                        position = Option::Some(tag::Position::Minor);
                    }
                    "p" => {
                        position = Option::Some(tag::Position::Patch);
                    }
                    _ => eprintln!("\n🛑 Invalid postion!"),
                }
            }
            new_tag = Option::Some(tags.first().unwrap().incremented(position.unwrap()));
        }
        let new_tag = new_tag.unwrap_or_else(|| panic!(""));
        println!("\n✅ New tag will be: {}.", new_tag.to_string());
        let input = self.prompt(&format!("🤖 Are you sure you want to add the new tag?:",))?;
        if !input.starts_with("y") {
            println!("canceled.");
        } else {
            println!("✨ Created the new tag: {}", new_tag.to_string());
        }
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
