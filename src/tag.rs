use anyhow::{anyhow, Context, Error, Result};
use semver::Version;
use std::str::FromStr;

#[derive(Debug)]
pub enum Position {
    Major,
    Minor,
    Patch,
}

impl FromStr for Position {
    type Err = Error;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "M" | "major" | "Major" => Ok(Self::Major),
            "m" | "minor" | "Minor" => Ok(Self::Minor),
            "p" | "patch" | "Patch" => Ok(Self::Patch),
            _ => Err(anyhow!(
                "Invalid position. Supported positions are major(M), minor(m), patch(p)."
            )),
        }
    }
}

#[derive(Debug, Eq, Clone)]
pub struct Tag {
    pub prefix: String,
    pub version: Version,
    pub timestamp: Option<i64>,
    pub message: Option<String>,
}

impl Tag {
    pub fn new(
        raw_version: &str,
        prefix: String,
        timestamp: Option<i64>,
        message: Option<String>,
    ) -> Result<Self> {
        let version = Version::parse(raw_version).with_context(|| {
            format!(
                "Version format error! Please input according to sematic versioning like '1.0.0'."
            )
        })?;
        Ok(Self {
            prefix: prefix,
            version: version,
            timestamp: timestamp,
            message: message,
        })
    }

    pub fn incremented(&self, position: Position) -> Self {
        let mut new_tag = self.clone();
        match position {
            Position::Major => new_tag.version.increment_major(),
            Position::Minor => new_tag.version.increment_minor(),
            Position::Patch => new_tag.version.increment_patch(),
        }
        new_tag
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        format!("{}{}", self.prefix, self.version.to_string())
    }
}

impl std::cmp::Ord for Tag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.version.cmp(&other.version)
    }
}

impl std::cmp::PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
