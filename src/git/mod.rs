use anyhow::Result;
use regex::Regex;
use std::process::Command;

use super::tag;

#[cfg(test)]
mod tests;

pub trait GitRepoIO {
    fn get_tags(&self, prefix: &str) -> Result<Vec<tag::Tag>>;
    fn get_log(&self, tag: Option<&tag::Tag>) -> Result<String>;
    fn add_tag(&self, tag: &tag::Tag) -> Result<()>;
}

pub struct GitRepo {
    client: Box<dyn GitClientIO>,
}

pub trait GitClientIO {
    fn get_tags(&self, prefix: &str) -> Result<String>;
    fn get_log(&self, tag: Option<&tag::Tag>) -> Result<String>;
    fn add_tag(&self, tag: &tag::Tag) -> Result<String>;
}

struct GitClient {}

impl GitClient {
    fn new() -> Self {
        Self {}
    }

    fn exec(&self, command: Vec<&str>) -> Result<String> {
        let output = Command::new("git").args(command).output()?;
        let stdout = String::from_utf8(output.stdout)?;
        Ok(stdout)
    }
}

impl GitClientIO for GitClient {
    fn get_tags(&self, prefix: &str) -> Result<String> {
        self.exec(vec![
            "tag",
            "-l",
            &format!("{}*", prefix),
            "--format='%(refname:short) %(creatordate:format:%s)'",
        ])
    }

    fn get_log(&self, tag: Option<&tag::Tag>) -> Result<String> {
        if tag.is_some() {
            let tag = tag.map(|t| t.to_string()).unwrap_or("".to_string());
            self.exec(vec!["log", "-n 1", "--stat", &tag])
        } else {
            self.exec(vec!["log", "-n 1", "--stat"])
        }
    }

    fn add_tag(&self, tag: &tag::Tag) -> Result<String> {
        match &tag.message {
            Some(message) => self.exec(vec!["tag", "-a", &tag.to_string(), "-m", message]),
            None => self.exec(vec!["tag", &tag.to_string()]),
        }
    }
}

impl GitRepo {
    pub fn new() -> Self {
        Self {
            client: Box::new(GitClient::new()),
        }
    }

    #[cfg(test)]
    fn new_with_client(client: Box<dyn GitClientIO>) -> Self {
        Self { client: client }
    }
}

impl GitRepoIO for GitRepo {
    /// run `git tag -l 'prefix*' --format='%(refname:short) %(creatordate:format:%s)'`
    /// => 'prefix1.0.0 1373529534'
    /// and convert reverse ordered Tag collection
    fn get_tags(&self, prefix: &str) -> Result<Vec<tag::Tag>> {
        let output = self.client.get_tags(prefix)?;
        let pattern = format!("'{}(.*) (.*)'", prefix);
        let regex = Regex::new(&pattern).unwrap();
        let mut tags: Vec<tag::Tag> = output
            .lines()
            .filter_map(|line| {
                let captures = regex.captures(line)?;
                let version = captures[1].to_string();
                let timestamp = captures[2].parse::<i64>().ok();
                tag::Tag::new(&version, prefix.to_string(), timestamp, None).ok()
            })
            .collect();
        tags.sort();
        tags.reverse();
        Ok(tags)
    }

    /// run `git log tag_name -n 1` and return response
    fn get_log(&self, tag: Option<&tag::Tag>) -> Result<String> {
        let log = self.client.get_log(tag)?;
        Ok(log)
    }

    /// run `git tag tag_name`
    fn add_tag(&self, tag: &tag::Tag) -> Result<()> {
        self.client.add_tag(tag)?;
        Ok(())
    }
}
