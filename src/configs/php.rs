use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[cfg_attr(
    feature = "config-schema",
    derive(schemars::JsonSchema),
    schemars(deny_unknown_fields)
)]
#[serde(default)]
pub struct PhpConfig<'a> {
    pub format: &'a str,
    pub version_format: &'a str,
    pub symbol: &'a str,
    pub style: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl Default for PhpConfig<'_> {
    fn default() -> Self {
        Self {
            format: "via [$symbol($version )]($style)",
            version_format: "v${raw}",
            symbol: "🐘 ",
            style: "147 bold",
            disabled: false,
            detect_extensions: vec!["php"],
            detect_files: vec!["composer.json", ".php-version"],
            detect_folders: vec![],
        }
    }
}
