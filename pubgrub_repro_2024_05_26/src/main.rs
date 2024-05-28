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
    fn make_ver_available(&mut self, n: PackageName, v: &str) {
        self.available_versions
            .entry(n)
            .or_default()
            .push(Version::parse(v).unwrap());
    }

    fn record_deps(&mut self, n: PackageName, v: &str, deps: &[(PackageName, &str)]) {
        self.dependencies.insert(
            (n, Version::parse(v).unwrap()),
            Dependencies::Known(Map::from_iter(deps.iter().map(|(dep, req)| {
                (*dep, Range::new(req.to_string()).to_pubgrub().unwrap())
            }))),
        );
    }

    pub fn new() -> Self {
        let mut this = Self {
            available_versions: HashMap::default(),
            dependencies: HashMap::default(),
        };

        this.make_ver_available("gleam_add_issue_2024_05_26", "0.0.0");
        this.make_ver_available("birl", "1.7.0");
        this.make_ver_available("gleam_javascript", "0.8.0");
        this.make_ver_available("gleam_community_colour", "1.4.0");
        this.make_ver_available("gleam_community_ansi", "1.4.0");
        this.make_ver_available("gleam_erlang", "0.25.0");
        this.make_ver_available("tom", "0.3.0");
        this.make_ver_available("glint", "1.0.0-rc2");
        this.make_ver_available("exception", "2.0.0");
        this.make_ver_available("ranger", "1.2.0");
        this.make_ver_available("filepath", "1.0.0");
        this.make_ver_available("startest", "0.2.4");
        this.make_ver_available("gleam_json", "1.0.1");
        this.make_ver_available("bigben", "1.0.0");
        this.make_ver_available("gleam_stdlib", "0.38.0");

        this.record_deps(
            "gleam_add_issue_2024_05_26",
            "0.0.0",
            &[
                ("bigben", ">=0.0.0"),
                ("gleam_stdlib", ">=0.0.0"),
                ("gleam_javascript", ">=0.0.0"),
                ("gleam_community_ansi", ">=0.0.0"),
                ("gleam_erlang", ">=0.0.0"),
                ("tom", ">=0.0.0"),
                ("glint", ">=0.0.0"),
                ("exception", ">=0.0.0"),
                ("filepath", ">=0.0.0"),
                ("startest", ">=0.0.0"),
                ("birl", ">=0.0.0"),
            ],
        );
        this.record_deps("gleam_stdlib", "0.38.0", &[]);
        this.record_deps("birl", "1.7.0", &[("ranger", ">=0.0.0")]);
        this.record_deps("bigben", "1.0.0", &[]);
        this.record_deps("gleam_javascript", "0.8.0", &[]);
        this.record_deps(
            "gleam_community_colour",
            "1.4.0",
            &[("gleam_json", ">=0.0.0")],
        );
        this.record_deps(
            "gleam_community_ansi",
            "1.4.0",
            &[("gleam_community_colour", ">=0.0.0")],
        );
        this.record_deps("gleam_erlang", "0.25.0", &[]);
        this.record_deps("tom", "0.3.0", &[]);
        this.record_deps("glint", "1.0.0-rc2", &[]);
        this.record_deps("exception", "2.0.0", &[]);
        this.record_deps("ranger", "1.2.0", &[]);
        this.record_deps("filepath", "1.0.0", &[]);
        this.record_deps("startest", "0.2.4", &[("glint", "<1.0.0-rc3")]);

        this
    }
}
