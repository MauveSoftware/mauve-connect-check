use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub name: String,
    #[serde(rename = "type")]
    pub type_name: String,
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
