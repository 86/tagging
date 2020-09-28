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
    #[structopt(short, long, default_value = "")]
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
        let tags = self.git.get_tags(&self.opt.prefix)?;
        let mut new_tag: tag::Tag;
        if tags.len() == 0 {
            loop {
                let tag = self.prompt(
                    "\n🤖 Tags based on semantic versioning does not exist yet. Please enter a new version:",
                )?;
                let result =
                    tag::Tag::new(&tag, self.opt.prefix.clone(), Option::None, Option::None);
                match result {
                    Ok(tag) => {
                        new_tag = tag;
                        break;
                    }
                    Err(err) => {
                        eprintln!("\n🛑 {}", err);
                        continue;
                    }
                }
            }
        } else {
            let pick = 3;
            let up_to = std::cmp::min(pick, tags.len());
            let latest_tags: String = tags[0..up_to]
                .iter()
                .enumerate()
                .map(|elem| {
                    let (index, tag) = elem;
                    let tag_name = tag.to_string();
                    let target_mark = if index == 0 { "  <-- 🎯 Target" } else { "" };
                    format!("{}{}", tag_name, target_mark)
                })
                .collect::<Vec<String>>()
                .join("\n");
            println!(
                "🔖 Latest tags:\n{}{}",
                latest_tags,
                if tags.len() > pick { "\n:" } else { "" }
            );
            loop {
                let input = self.prompt(
                    "\n🤖 Which position do you want to increment?\nmajor(M) / minor(m) / patch(p):",
                )?;
                let position: tag::Position;
                match input.as_str() {
                    "M" => position = tag::Position::Major,
                    "m" => position = tag::Position::Minor,
                    "p" => position = tag::Position::Patch,
                    _ => {
                        eprintln!("\n🛑 Invalid position!");
                        continue;
                    }
                }
                new_tag = tags.first().unwrap().incremented(position);
                break;
            }
        }
        let commit_info = self.git.get_log(None)?;
        println!(
            "\n✅ The new tag will be: {}.\n{}",
            new_tag.to_string(),
            commit_info
        );
        let message = self.confirm_tag_message()?;
        new_tag.message = message;
        let input = self.prompt(&format!(
            "\n🤖 Are you sure you want to add the new tag?: (y/n)",
        ))?;
        if !input.starts_with("y") {
            println!("\n❌ Canceled.");
        } else {
            self.git.add_tag(&new_tag)?;
            println!("\n✨ Created the new tag: {} ✨", new_tag.to_string());
            println!("\n✅ Done.");
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

    fn confirm_tag_message(&self) -> std::io::Result<Option<String>> {
        self.prompt("\n🤖 Enter the annotation message if needed: (only return key to skip)")
            .map(|m| {
                if m.len() == 0 {
                    Option::None
                } else {
                    Option::Some(m)
                }
            })
    }
}
