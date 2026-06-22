//! Interacts with crates.io to fetch crate information.

use serde::Deserialize;
use ureq;

#[derive(Deserialize)]
pub struct FullCrateInfo {
    #[serde(rename = "crate")]
    pub crate_data: Crate,

    // Explicitly not including the versions or keywords or categories.
    // Could use 'include=default_version' to pull in the Version structure for just the
    // version marked in the crate 'default_version' field.
}

#[derive(Deserialize)]
pub struct Crate {
    pub name: String,
    pub default_version: String,
}


pub fn read_crate_info(crate_name: &str) -> Result<FullCrateInfo, ureq::Error> {
    let url = format!(
        "https://crates.io/api/v1/crates/{}?include=",
        crate_name,
    );
    // should be able to use .body_mut().read_json(), but it's generating a compile error.
    let res = ureq::get(url).call()?.body_mut().read_to_string()?;
    return Ok(serde_json::from_str::<FullCrateInfo>(&res).unwrap());
}
