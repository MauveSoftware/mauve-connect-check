use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Kind {
    A,
    AAAA,
    CNAME,
}

impl Kind {
    pub fn as_str(&self) -> &str {
        match self {
            Kind::A => "A",
            Kind::AAAA => "AAAA",
            Kind::CNAME => "CNAME",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: String,
    #[serde(rename = "type")]
    pub kind: Kind,
    pub actual: Vec<String>,
    pub expected: Vec<String>,
    pub passed: bool,
    pub incomplete: bool,
    pub legacy: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckResult {
    pub success: bool,
    pub records: Vec<Record>,
}
