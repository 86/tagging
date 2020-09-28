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
        let mut new_tag = self.confirm_new_tag()?;
        self.show_commit_info(&new_tag)?;
        new_tag.message = self.confirm_tag_message()?;
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

    fn confirm_new_tag(&self) -> Result<tag::Tag> {
        let new_tag: tag::Tag;
        let tags = self.git.get_tags(&self.opt.prefix)?;
        if tags.len() > 0 {
            self.list_latest_tag(&tags, 3);
            let base_tag = tags.first().unwrap();
            new_tag = self.request_incremented_tag(&base_tag)?
        } else {
            new_tag = self.request_initial_tag()?
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

    fn request_incremented_tag(&self, base_tag: &tag::Tag) -> Result<tag::Tag> {
        let new_tag: tag::Tag;
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
            new_tag = base_tag.incremented(position);
            break;
        }
        Ok(new_tag)
    }

    fn request_initial_tag(&self) -> Result<tag::Tag> {
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
