use std::cmp::Ordering;
use std::fmt::{Display, format, Formatter};
use std::num::ParseIntError;
use chrono::Utc;
use regex::Regex;

#[derive(Debug, Eq)]
pub struct DwVersion {
    has_prefix: bool,
    major: u64,
    minor: u64,
    patch: u64,
    pre: Option<String>,
    build: Option<u64>,
}

impl DwVersion {
    pub fn parse(param_version: &str) -> Option<DwVersion> {
        println!("param_version:{}", param_version);
        let mut version_suffix = param_version.split('_');
        let version = version_suffix.next().unwrap();
        let mut version = version.split('.');
        let major_prefix = version.next().unwrap();
        let major;
        let has_prefix = major_prefix.contains("v");
        if has_prefix {
            major = match major_prefix.replace("v", "").parse::<u64>() {
                Ok(major) => major,
                Err(_) => 0
            };
        } else {
            major = match major_prefix.parse::<u64>() {
                Ok(major) => major,
                Err(_) => 0
            };
        }
        let minor = match version.next().unwrap_or("0").parse::<u64>() {
            Ok(minor) => minor,
            Err(_) => 0
        };
        let patch = match version.next().unwrap_or("0").parse::<u64>() {
            Ok(patch) => patch,
            Err(_) => 0
        };
        let next = version_suffix.next();
        if next.is_none() {
            return Some(DwVersion {
                has_prefix,
                major,
                minor,
                patch,
                pre: None,
                build: None,
            });
        }
        let build = next.unwrap().parse::<u64>().unwrap();
        let pre = version_suffix.next().unwrap();
        Some(DwVersion {
            has_prefix,
            major,
            minor,
            patch,
            pre: Some(format!("{}", pre)),
            build: Some(build),
        })
    }
    pub fn is_valid_version(version: &str) -> bool {
        let version_regex = Regex::new(r"^v?\d+\.\d+\.\d+(\w*)?$").unwrap();
        version_regex.is_match(version)
    }
    pub fn plus_major(&self) -> DwVersion {
        Self {
            has_prefix: self.has_prefix,
            major: self.major + 1,
            minor: self.minor,
            patch: self.patch,
            pre: self.pre.clone(),
            build: self.build,
        }
    }
    pub fn plus_minor(&self) -> DwVersion {
        Self {
            has_prefix: self.has_prefix,
            major: self.major,
            minor: self.minor + 1,
            patch: self.patch,
            pre: self.pre.clone(),
            build: self.build,
        }
    }

    pub fn plus_patch(&mut self) -> DwVersion {
        Self {
            has_prefix: self.has_prefix,
            major: self.major,
            minor: self.minor,
            patch: self.patch + 1,
            pre: self.pre.clone(),
            build: self.build,
        }
    }
    pub fn set_pre(&mut self, pre: String) {
        self.pre = Some(pre);
    }
    pub fn set_build(&mut self, build: u64) {
        self.build = Some(build);
    }
    pub fn auto_set_build(&mut self) {
        // 获取当前日期
        let today = Utc::now();
        let date_str = today.format("%Y%m%d").to_string();
        self.build = Some(date_str.parse::<u64>().unwrap());
    }
    pub fn default() -> DwVersion {
        DwVersion {
            has_prefix: true,
            major: 1,
            minor: 0,
            patch: 0,
            pre: None,
            build: None,
        }
    }
}

impl Display for DwVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut version = String::new();
        if self.has_prefix {
            version.push_str("v");
        }
        version.push_str(self.major.to_string().as_str());
        version.push_str(".");
        version.push_str(self.minor.to_string().as_str());
        version.push_str(".");
        version.push_str(self.patch.to_string().as_str());
        if self.build.is_some() {
            version.push_str("_");
            version.push_str(self.build.unwrap().to_string().as_str());
        }
        if self.pre.is_some() {
            version.push_str("_");
            version.push_str(&self.pre.clone().unwrap());
        }
        write!(f, "{}", version)
    }
}

impl From<&str> for DwVersion {
    fn from(param_version: &str) -> Self {
        if !DwVersion::is_valid_version(param_version) {
            panic!("invalid version:{}", param_version);
        }
        DwVersion::parse(param_version).unwrap()
    }
}

impl Ord for DwVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major != other.major {
            return self.major.cmp(&other.major);
        }
        if self.minor != other.minor {
            return self.minor.cmp(&other.minor);
        }
        if self.patch != other.patch {
            return self.patch.cmp(&other.patch);
        }
        Ordering::Equal
    }
}

impl PartialEq<Self> for DwVersion {
    fn eq(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch == other.patch
    }
}

impl PartialOrd for DwVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod testing {
    use crate::version::DwVersion;

    #[test]
    pub fn test_version() {
        let version = DwVersion::parse("v1.0.5_20231126_Release").unwrap();
        println!("major:{}", version.major);
        println!("minor:{}", version.minor);
        println!("patch:{}", version.patch);
        println!("pre:{:?}", version.pre);
        println!("BuildMetadata:{:?}", version.build);
    }

    #[test]
    pub fn test_version1() {
        let mut version = DwVersion::parse("v1.0.0").unwrap();
        println!("major:{}", version.major);
        println!("minor:{}", version.minor);
        println!("patch:{}", version.patch);
        println!("pre:{:?}", version.pre);
        println!("BuildMetadata:{:?}", version.build);
        version = version.plus_patch();
        version.set_pre("beta".to_string());
        version.set_build(20231126);
        println!("version:{}", version);
    }

    #[test]
    pub fn is_valid_version() {
        let version = DwVersion::is_valid_version("v1.0.0g");
        assert!(version, "v1.0.0 is valid version");
    }

    #[test]
    pub fn is_valid_version1() {
        let version = DwVersion::is_valid_version("v1.0.0_20150327_release");
        assert!(version, "v1.0.0_20150327_release is valid version");
    }
}