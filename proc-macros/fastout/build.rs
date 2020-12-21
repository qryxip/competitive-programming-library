use wattbuild::{Dependency, Source};

fn main() {
    wattbuild::build(
        &[Dependency {
            package: "fastout_impl",
            source: Source::Path {
                path: "./impl",
                or: Some(Box::new(Source::Git {
                    git: "https://github.com/qryxip/oj-verify-playground",
                    rev: None,
                })),
            },
        }],
        None,
        None,
    );
}