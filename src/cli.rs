use anyhow::{anyhow, Result};
use std::str::FromStr;
use structopt::StructOpt;

use super::git::{GitRepo, GitRepoIO};
use super::tag;

#[derive(StructOpt, Debug)]
#[structopt(name = "tagging")]
pub struct Opt {
    /// Disable prompt
    #[structopt(short, long)]
    pub quiet: bool,

    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Specify increment position
    #[structopt(short, long)]
    pub inc_position: Option<tag::Position>,

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
        if self.opt.quiet && self.opt.inc_position.is_none() {
            return Err(anyhow!(
                "🛑 Needs to specify `inc_position` if you use `quiet` flag"
            ));
        }
        let mut new_tag = self.new_tag()?;
        self.show_commit_info(&new_tag)?;
        if !self.opt.quiet {
            new_tag.message = self.confirm_tag_message()?;
        }
        self.add_new_tag(&new_tag)?;
        Ok(())
    }

    fn prompt(&self, description: &str) -> Result<String> {
        println!("{}", description);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input.pop();
        Ok(input)
    }

    fn new_tag(&self) -> Result<tag::Tag> {
        if !self.opt.quiet {
            return self.confirm_new_tag();
        }
        let new_tag: tag::Tag;
        let position = self.opt.inc_position.clone().unwrap();
        let tags = self.git.get_tags(&self.opt.prefix)?;
        if tags.len() > 0 {
            let base_tag = tags.first().unwrap();
            new_tag = base_tag.incremented(position);
        } else {
            let raw_tag: &str;
            match position {
                tag::Position::Major => raw_tag = "1.0.0",
                tag::Position::Minor => raw_tag = "0.1.0",
                tag::Position::Patch => raw_tag = "0.0.1",
            }
            let prefix = self.opt.prefix.clone();
            new_tag = tag::Tag::new(raw_tag, prefix, None, None).unwrap();
        }
        Ok(new_tag)
    }

    fn confirm_new_tag(&self) -> Result<tag::Tag> {
        let new_tag: tag::Tag;
        let tags = self.git.get_tags(&self.opt.prefix)?;
        if tags.len() > 0 {
            self.list_latest_tag(&tags, 3);
            let base_tag = tags.first().unwrap();
            new_tag = self.prompt_incremented_tag(&base_tag)?
        } else {
            new_tag = self.prompt_initial_tag()?
        }
        Ok(new_tag)
    }

    fn list_latest_tag(&self, tags: &Vec<tag::Tag>, pick: usize) {
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
        )
    }

    fn prompt_incremented_tag(&self, base_tag: &tag::Tag) -> Result<tag::Tag> {
        let new_tag: tag::Tag;
        loop {
            let input = self.prompt(
                "\n🤖 Which position do you want to increment?\nmajor(M) / minor(m) / patch(p):",
            )?;
            let position: tag::Position;
            match tag::Position::from_str(input.as_str()) {
                Ok(value) => position = value,
                Err(err) => {
                    eprintln!("\n🛑 {}", err);
                    continue;
                }
            }
            new_tag = base_tag.incremented(position);
            break;
        }
        Ok(new_tag)
    }

    fn prompt_initial_tag(&self) -> Result<tag::Tag> {
        let initial_tag: tag::Tag;
        loop {
            let tag = self.prompt(
                "\n🤖 Tags based on semantic versioning does not exist yet. Please enter a new version:",
            )?;
            let result = tag::Tag::new(&tag, self.opt.prefix.clone(), None, None);
            match result {
                Ok(tag) => {
                    initial_tag = tag;
                    break;
                }
                Err(err) => {
                    eprintln!("\n🛑 {}", err);
                    continue;
                }
            }
        }
        Ok(initial_tag)
    }

    fn show_commit_info(&self, new_tag: &tag::Tag) -> Result<()> {
        let commit_info = self.git.get_log(None)?;
        println!(
            "\n✅ The new tag will be: {}.\n{}",
            new_tag.to_string(),
            commit_info
        );
        Ok(())
    }

    fn confirm_tag_message(&self) -> Result<Option<String>> {
        self.prompt("\n🤖 Enter the annotation message if needed: (only return key to skip)")
            .map(|m| if m.len() == 0 { None } else { Some(m) })
    }

    fn add_new_tag(&self, new_tag: &tag::Tag) -> Result<()> {
        self.git.add_tag(new_tag)?;
        println!("\n✨ Created the new tag: {} ✨", new_tag.to_string());
        println!("\n✅ Done.");
        Ok(())
    }
}
