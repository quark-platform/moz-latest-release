use std::convert::TryFrom;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct FirefoxVersionAPI {
    pub FIREFOX_AURORA: String,
    pub FIREFOX_DEVEDITION: String,
    pub FIREFOX_ESR: String,
    pub FIREFOX_ESR_NEXT: String,
    pub FIREFOX_NIGHTLY: String,
    pub LAST_MERGE_DATE: String,
    pub LAST_RELEASE_DATE: String,
    pub LATEST_FIREFOX_VERSION: String,
    pub LATEST_FIREFOX_DEVEL_VERSION: String,
}

impl FirefoxVersionAPI {
    pub fn get_version<'a>(&'a self, target: &FirefoxTargets) -> &'a str {
        match target {
            FirefoxTargets::Stable => &self.LATEST_FIREFOX_VERSION,
            FirefoxTargets::Beta => &self.LATEST_FIREFOX_DEVEL_VERSION,
            FirefoxTargets::Nightly => &self.FIREFOX_NIGHTLY,
            FirefoxTargets::DevEdition => &self.FIREFOX_DEVEDITION,
            FirefoxTargets::ESRNext => &self.FIREFOX_ESR_NEXT,
        }
    }
}

pub enum FirefoxTargets {
    Stable,
    Beta,
    Nightly,
    DevEdition,
    ESRNext,
}

impl TryFrom<&String> for FirefoxTargets {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "stable" => Ok(FirefoxTargets::Stable),
            "beta" => Ok(FirefoxTargets::Beta),
            "nightly" => Ok(FirefoxTargets::Nightly),
            "dev" => Ok(FirefoxTargets::DevEdition),
            "esr" => Ok(FirefoxTargets::ESRNext),
            _ => Err(Box::leak(
                // TODO: Don't create a memory leak
                format!(
                    "Invalid target '{}'. Try 'stable', 'beta', 'nightly', 'dev', or 'esr'.",
                    value
                )
                .into_boxed_str(),
            )),
        }
    }
}

impl FirefoxTargets {
    pub async fn get_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        let resp: FirefoxVersionAPI =
            reqwest::get("https://product-details.mozilla.org/1.0/firefox_versions.json")
                .await?
                .json()
                .await?;

        Ok(resp.get_version(self).to_string())
    }
}

pub async fn get_version_from_target(
    target: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let target = FirefoxTargets::try_from(target)?;
    target.get_version().await
}

pub fn get_source_url(version: String) -> String {
    format!(
        "https://archive.mozilla.org/pub/firefox/releases/{}/source/firefox-{}.source.tar.xz",
        version, version
    )
}
