use anyhow::Result;

use super::tag;
use super::{GitClientIO, GitRepo, GitRepoIO};

struct MockGitClient {
    output: String,
}

impl MockGitClient {
    fn new(output: &str) -> Self {
        Self {
            output: output.to_string(),
        }
    }
}

impl GitClientIO for MockGitClient {
    fn get_tags(&self, _prefix: &str) -> Result<String> {
        Ok(self.output.to_string())
    }

    fn get_log(&self, _tag: Option<&tag::Tag>) -> Result<String> {
        Ok(self.output.to_string())
    }

    fn add_tag(&self, _tag: &tag::Tag) -> Result<String> {
        Ok(self.output.to_string())
    }

    fn push_tag(&self, _tag: &tag::Tag) -> Result<String> {
        Ok(self.output.to_string())
    }
}

#[test]
fn test_get_tags_none() {
    let mock = MockGitClient::new("");
    let git = GitRepo::new_with_client(Box::new(mock));
    let tags = git.get_tags("").unwrap();
    assert_eq!(tags.len(), 0);
}

#[test]
fn test_get_tags_non_prefix() {
    let output = "
    '1.0.0 1600000000'
    '1.1.0 1600000000'
    '1.0.1 1600000000'
    ";
    let mock = MockGitClient::new(output);
    let git = GitRepo::new_with_client(Box::new(mock));
    let tags = git.get_tags("").unwrap();
    assert_eq!(tags.len(), 3);
    let tags: Vec<String> = tags.iter().map(|t| t.to_string()).collect();
    assert_eq!(tags, vec!["1.1.0", "1.0.1", "1.0.0",]);
}

#[test]
fn test_get_tags_with_prefix() {
    let output = "
    'v1.0.0 1600000000'
    'v1.1.0 1600000000'
    '1.0.1 1600000000'
    ";
    let mock = MockGitClient::new(output);
    let git = GitRepo::new_with_client(Box::new(mock));
    let tags = git.get_tags("v").unwrap();
    assert_eq!(tags.len(), 2);
    let tags: Vec<String> = tags.iter().map(|t| t.to_string()).collect();
    assert_eq!(tags, vec!["v1.1.0", "v1.0.0"]);
}
