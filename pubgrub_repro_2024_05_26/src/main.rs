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

type PackageName = &'static str;

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
            |name| {
                self.available_versions
                    .get(name)
                    .into_iter()
                    .flatten()
                    .cloned()
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
            .get(&(name, version.clone()))
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
            .entry("gleam_add_issue_2024_05_26")
            .or_default()
            .push(Version::parse("0.0.0").unwrap());
        this.available_versions
            .entry("argv")
            .or_default()
            .push(Version::parse("1.0.2").unwrap());
        this.available_versions
            .entry("birl")
            .or_default()
            .push(Version::parse("1.7.0").unwrap());
        this.available_versions
            .entry("gleam_javascript")
            .or_default()
            .push(Version::parse("0.8.0").unwrap());
        this.available_versions
            .entry("gleam_community_colour")
            .or_default()
            .push(Version::parse("1.4.0").unwrap());
        this.available_versions
            .entry("gleam_community_ansi")
            .or_default()
            .push(Version::parse("1.4.0").unwrap());
        this.available_versions
            .entry("gleam_erlang")
            .or_default()
            .push(Version::parse("0.25.0").unwrap());
        this.available_versions
            .entry("tom")
            .or_default()
            .push(Version::parse("0.3.0").unwrap());
        this.available_versions
            .entry("thoas")
            .or_default()
            .push(Version::parse("1.2.1").unwrap());
        this.available_versions
            .entry("glint")
            .or_default()
            .push(Version::parse("1.0.0-rc2").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.14.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.13.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.12.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.11.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.10.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.9.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.8.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.7.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.6.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.5.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.4.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.3.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.2.0").unwrap());
        this.available_versions
            .entry("wisp")
            .or_default()
            .push(Version::parse("0.1.0").unwrap());
        this.available_versions
            .entry("snag")
            .or_default()
            .push(Version::parse("0.3.0").unwrap());
        this.available_versions
            .entry("gleam_otp")
            .or_default()
            .push(Version::parse("0.10.0").unwrap());
        this.available_versions
            .entry("exception")
            .or_default()
            .push(Version::parse("2.0.0").unwrap());
        this.available_versions
            .entry("ranger")
            .or_default()
            .push(Version::parse("1.2.0").unwrap());
        this.available_versions
            .entry("simplifile")
            .or_default()
            .push(Version::parse("1.7.0").unwrap());
        this.available_versions
            .entry("filepath")
            .or_default()
            .push(Version::parse("1.0.0").unwrap());
        this.available_versions
            .entry("startest")
            .or_default()
            .push(Version::parse("0.2.4").unwrap());
        this.available_versions
            .entry("gleam_json")
            .or_default()
            .push(Version::parse("1.0.1").unwrap());
        this.available_versions
            .entry("bigben")
            .or_default()
            .push(Version::parse("1.0.0").unwrap());
        this.available_versions
            .entry("gleam_stdlib")
            .or_default()
            .push(Version::parse("0.38.0").unwrap());

        this.dependencies.insert(
            (
                "gleam_add_issue_2024_05_26",
                Version::parse("0.0.0").unwrap(),
            ),
            Dependencies::Known(Map::from_iter([
                (
                    "bigben",
                    Range::new("1.0.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new("0.38.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_javascript",
                    Range::new("0.8.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_community_colour",
                    Range::new("1.4.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_community_ansi",
                    Range::new("1.4.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_erlang",
                    Range::new("0.25.0".to_string()).to_pubgrub().unwrap(),
                ),
                ("tom", Range::new("0.3.0".to_string()).to_pubgrub().unwrap()),
                (
                    "thoas",
                    Range::new("1.2.1".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "glint",
                    Range::new("1.0.0-rc2".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "wisp",
                    pubgrub::range::Range::any(),
                    // Range::new("*".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "snag",
                    Range::new("0.3.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_otp",
                    Range::new("0.10.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "simplifile",
                    Range::new("1.7.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "ranger",
                    Range::new("1.2.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "exception",
                    Range::new("2.0.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "filepath",
                    Range::new("1.0.0".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "gleam_json",
                    Range::new("1.0.1".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "startest",
                    Range::new("0.2.4".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "argv",
                    Range::new("1.0.2".to_string()).to_pubgrub().unwrap(),
                ),
                (
                    "birl",
                    Range::new("1.7.0".to_string()).to_pubgrub().unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("gleam_stdlib", Version::parse("0.38.0").unwrap()),
            Dependencies::Known(Map::from_iter([])),
        );
        this.dependencies.insert(
            ("argv", Version::parse("1.0.2").unwrap()),
            Dependencies::Known(Map::from_iter([])),
        );
        this.dependencies.insert(
            ("birl", Version::parse("1.7.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_stdlib",
                    Range::new(">= 0.37.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "ranger",
                    Range::new(">= 1.2.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("bigben", Version::parse("1.0.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_otp",
                    Range::new(">= 0.10.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new(">= 0.34.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_erlang",
                    Range::new(">= 0.25.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "birl",
                    Range::new(">= 1.6.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("gleam_javascript", Version::parse("0.8.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.19.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("gleam_community_colour", Version::parse("1.4.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_stdlib",
                    Range::new(">= 0.34.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_json",
                    Range::new(">= 0.7.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("gleam_community_ansi", Version::parse("1.4.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_community_colour",
                    Range::new(">= 1.3.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new(">= 0.34.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("gleam_erlang", Version::parse("0.25.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.33.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("tom", Version::parse("0.3.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.33.0 and < 1.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("thoas", Version::parse("1.2.1").unwrap()),
            Dependencies::Known(Map::from_iter([])),
        );
        this.dependencies.insert(
            ("glint", Version::parse("1.0.0-rc2").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_community_colour",
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_community_ansi",
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "snag",
                    Range::new(">= 0.3.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new(">= 0.36.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("snag", Version::parse("0.3.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.34.0 and < 1.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("gleam_otp", Version::parse("0.10.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "gleam_erlang",
                    Range::new(">= 0.22.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new(">= 0.32.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("exception", Version::parse("2.0.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.30.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("ranger", Version::parse("1.2.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.36.0 and < 2.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("simplifile", Version::parse("1.7.0").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "filepath",
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new(">= 0.34.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );
        this.dependencies.insert(
            ("filepath", Version::parse("1.0.0").unwrap()),
            Dependencies::Known(Map::from_iter([(
                "gleam_stdlib",
                Range::new(">= 0.32.0 and < 1.0.0".to_string())
                    .to_pubgrub()
                    .unwrap(),
            )])),
        );
        this.dependencies.insert(
            ("startest", Version::parse("0.2.4").unwrap()),
            Dependencies::Known(Map::from_iter([
                (
                    "argv",
                    Range::new(">= 1.0.2 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_stdlib",
                    Range::new(">= 0.36.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "exception",
                    Range::new(">= 2.0.0 and < 3.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "simplifile",
                    Range::new(">= 1.7.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_javascript",
                    Range::new(">= 0.8.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_community_ansi",
                    Range::new(">= 1.4.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "gleam_erlang",
                    Range::new(">= 0.25.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "tom",
                    Range::new(">= 0.3.0 and < 1.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "glint",
                    Range::new(">= 1.0.0-rc2 and < 1.0.0-rc3".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "bigben",
                    Range::new(">= 1.0.0 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
                (
                    "birl",
                    Range::new(">= 1.6.1 and < 2.0.0".to_string())
                        .to_pubgrub()
                        .unwrap(),
                ),
            ])),
        );

        this
    }
}
