use std::{borrow::Borrow, collections::HashMap, error::Error as StdError};

use hexpm::version::{Range, Version};
use pubgrub::{
    solver::{choose_package_with_fewest_versions, Dependencies},
    type_aliases::Map,
};

fn main() {
    let dependency_provider = Issue3201DependencyProvider::new();

    let result = pubgrub::solver::resolve(
        &dependency_provider,
        "gleam_add_issue_2024_05_26".into(),
        Version::new(0, 0, 0),
    );

    dbg!(&result);

    assert!(result.is_ok());
}

type PackageName = String;

type PubgrubRange = pubgrub::range::Range<Version>;

struct Issue3201DependencyProvider {
    available_versions: HashMap<PackageName, Vec<Version>>,
    dependencies: HashMap<(PackageName, Version), Dependencies<PackageName, Version>>,
}

impl pubgrub::solver::DependencyProvider<PackageName, Version> for Issue3201DependencyProvider {
    fn choose_package_version<Name: Borrow<PackageName>, Ver: Borrow<PubgrubRange>>(
        &self,
        potential_packages: impl Iterator<Item = (Name, Ver)>,
    ) -> Result<(Name, Option<Version>), Box<dyn StdError>> {
        Ok(choose_package_with_fewest_versions(
            |name: &String| {
                let Some(available_versions) = self.available_versions.get(name) else {
                    return Vec::new().into_iter();
                };

                available_versions.clone().into_iter()
            },
            potential_packages.into_iter(),
        ))
    }

    fn get_dependencies(
        &self,
        name: &PackageName,
        version: &Version,
    ) -> Result<Dependencies<PackageName, Version>, Box<dyn StdError>> {
        self.dependencies
            .get(&(name.clone(), version.clone()))
            .cloned()
            .ok_or_else(|| "failed to get dependencies".into())
    }
}

impl Issue3201DependencyProvider {
    pub fn new() -> Self {
        let mut this = Self {
            available_versions: HashMap::default(),
            dependencies: HashMap::default(),
        };

        this.available_versions
            .entry("gleam_add_issue_2024_05_26".to_string())
            .or_default()
            .push(Version::parse("0.0.0").unwrap());
        this.available_versions
            .entry("argv".to_string())
            .or_default()
            .push(Version::parse("1.0.2").unwrap());
        this.available_versions
            .entry("birl".to_string())
            .or_default()
            .push(Version::parse("1.7.0").unwrap());
        this.available_versions
            .entry("gleam_javascript".to_string())
            .or_default()
            .push(Version::parse("0.8.0").unwrap());
        this.available_versions
            .entry("gleam_community_colour".to_string())
            .or_default()
            .push(Version::parse("1.4.0").unwrap());
        this.available_versions
            .entry("gleam_community_ansi".to_string())
            .or_default()
            .push(Version::parse("1.4.0").unwrap());
        this.available_versions
            .entry("gleam_erlang".to_string())
            .or_default()
            .push(Version::parse("0.25.0").unwrap());
        this.available_versions
            .entry("tom".to_string())
            .or_default()
            .push(Version::parse("0.3.0").unwrap());
        this.available_versions
            .entry("thoas".to_string())
            .or_default()
            .push(Version::parse("1.2.1").unwrap());
        this.available_versions
            .entry("glint".to_string())
            .or_default()
            .push(Version::parse("1.0.0-rc2").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.14.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.13.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.12.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.11.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.10.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.9.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.8.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.7.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.6.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.5.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.4.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.3.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.2.0").unwrap());
        this.available_versions
            .entry("wisp".to_string())
            .or_default()
            .push(Version::parse("0.1.0").unwrap());
        this.available_versions
            .entry("snag".to_string())
            .or_default()
            .push(Version::parse("0.3.0").unwrap());
        this.available_versions
            .entry("gleam_otp".to_string())
            .or_default()
            .push(Version::parse("0.10.0").unwrap());
        this.available_versions
            .entry("exception".to_string())
            .or_default()
            .push(Version::parse("2.0.0").unwrap());
        this.available_versions
            .entry("ranger".to_string())
            .or_default()
            .push(Version::parse("1.2.0").unwrap());
        this.available_versions
            .entry("simplifile".to_string())
            .or_default()
            .push(Version::parse("1.7.0").unwrap());
        this.available_versions
            .entry("filepath".to_string())
            .or_default()
            .push(Version::parse("1.0.0").unwrap());
        this.available_versions
            .entry("startest".to_string())
            .or_default()
            .push(Version::parse("0.2.4").unwrap());
        this.available_versions
            .entry("gleam_json".to_string())
            .or_default()
            .push(Version::parse("1.0.1").unwrap());
        this.available_versions
            .entry("bigben".to_string())
            .or_default()
            .push(Version::parse("1.0.0").unwrap());
        this.available_versions
            .entry("gleam_stdlib".to_string())
            .or_default()
            .push(Version::parse("0.38.0").unwrap());

        let _ = this.dependencies.insert(
            (
                "gleam_add_issue_2024_05_26".to_string(),
                Version::parse("0.0.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([
                (
                    "bigben".to_string(),
                    Range::new("1.0.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new("0.38.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_javascript".to_string(),
                    Range::new("0.8.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_community_colour".to_string(),
                    Range::new("1.4.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_community_ansi".to_string(),
                    Range::new("1.4.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_erlang".to_string(),
                    Range::new("0.25.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "tom".to_string(),
                    Range::new("0.3.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "thoas".to_string(),
                    Range::new("1.2.1".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "glint".to_string(),
                    Range::new("1.0.0-rc2".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "wisp".to_string(),
                    pubgrub::range::Range::any(),
                    // Range::new("*".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "snag".to_string(),
                    Range::new("0.3.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_otp".to_string(),
                    Range::new("0.10.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "simplifile".to_string(),
                    Range::new("1.7.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "ranger".to_string(),
                    Range::new("1.2.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "exception".to_string(),
                    Range::new("2.0.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "filepath".to_string(),
                    Range::new("1.0.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_json".to_string(),
                    Range::new("1.0.1".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "startest".to_string(),
                    Range::new("0.2.4".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "argv".to_string(),
                    Range::new("1.0.2".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "birl".to_string(),
                    Range::new("1.7.0".to_string()).to_pubgrub().unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            (
                "gleam_stdlib".to_string(),
                Version::parse("0.38.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([])),
        );
        let _ = this.dependencies.insert(
            ("argv".to_string(), Version::parse("1.0.2").unwrap()),
            Dependencies::Known(Map::from_iter([])),
        );
        let _ = this.dependencies.insert(
            ("birl".to_string(), Version::parse("1.7.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.37.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "ranger".to_string(),
                    Range::new(">= 1.2.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            ("bigben".to_string(), Version::parse("1.0.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_otp".to_string(),
                    Range::new(">= 0.10.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.34.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_erlang".to_string(),
                    Range::new(">= 0.25.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "birl".to_string(),
                    Range::new(">= 1.6.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            (
                "gleam_javascript".to_string(),
                Version::parse("0.8.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.19.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            (
                "gleam_community_colour".to_string(),
                Version::parse("1.4.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.34.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_json".to_string(),
                    Range::new(">= 0.7.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            (
                "gleam_community_ansi".to_string(),
                Version::parse("1.4.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_community_colour".to_string(),
                    Range::new(">= 1.3.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.34.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            (
                "gleam_erlang".to_string(),
                Version::parse("0.25.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.33.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            ("tom".to_string(), Version::parse("0.3.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.33.0 and < 1.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            ("thoas".to_string(), Version::parse("1.2.1").unwrap()),
            Dependencies::Known(Map::from_iter([])),
        );
        let _ = this.dependencies.insert(
            ("glint".to_string(), Version::parse("1.0.0-rc2").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_community_colour".to_string(),
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_community_ansi".to_string(),
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "snag".to_string(),
                    Range::new(">= 0.3.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.36.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            ("snag".to_string(), Version::parse("0.3.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.34.0 and < 1.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            ("gleam_otp".to_string(), Version::parse("0.10.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_erlang".to_string(),
                    Range::new(">= 0.22.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.32.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            ("exception".to_string(), Version::parse("2.0.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.30.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            ("ranger".to_string(), Version::parse("1.2.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.36.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            ("simplifile".to_string(), Version::parse("1.7.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "filepath".to_string(),
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.34.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        let _ = this.dependencies.insert(
            ("filepath".to_string(), Version::parse("1.0.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib".to_string(),
                Range::new(">= 0.32.0 and < 1.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        let _ = this.dependencies.insert(
            ("startest".to_string(), Version::parse("0.2.4").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "argv".to_string(),
                    Range::new(">= 1.0.2 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib".to_string(),
                    Range::new(">= 0.36.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "exception".to_string(),
                    Range::new(">= 2.0.0 and < 3.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "simplifile".to_string(),
                    Range::new(">= 1.7.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_javascript".to_string(),
                    Range::new(">= 0.8.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_community_ansi".to_string(),
                    Range::new(">= 1.4.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_erlang".to_string(),
                    Range::new(">= 0.25.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "tom".to_string(),
                    Range::new(">= 0.3.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "glint".to_string(),
                    Range::new(">= 1.0.0-rc2 and < 1.0.0-rc3".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "bigben".to_string(),
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "birl".to_string(),
                    Range::new(">= 1.6.1 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );

        this
    }
}
