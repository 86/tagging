use semver::Version;

pub struct Tag {
    pub prefix: String,
    pub version: Version,
}

impl Tag {
    pub fn new(raw_version: &str, prefix: String) -> Result<Self, Box<dyn std::error::Error>> {
        let version = Version::parse(raw_version)?;
        Ok(Self {
            prefix: prefix,
            version: version,
        })
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        format!("{}{}", self.prefix, self.version.to_string())
    }
}
