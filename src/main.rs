use ignore_files::{IgnoreFile, IgnoreFilter};
use std::path::{Path, PathBuf};

// Config
/*
    Truth table:
    | INCLUDE_ROOT_IGNORE_FILE | USE_PREFIXED_PATH_AS_APPLIES_IN_PARAMETER | Outcome                            |
    | false                    | false                                     | `add_globs` will have no effect    |
    | false                    | true                                      | `add_globs` works as intended      |
    | true                     | false                                     | `add_globs` works as intended      |
    | true                     | true                                      | `add_globs` works as intended      |
*/
/// Whether the root ignore file `test.ignore` should be passed to the IgnoreFilter constructor.
const INCLUDE_ROOT_IGNORE_FILE: bool = true;
/// Whether the `applies_in` param of `add_globs()` should be passed as `prefix(cwd)` or `cwd`.
const USE_PREFIXED_PATH_AS_APPLIES_IN_PARAMETER: bool = true;

#[tokio::main]
async fn main() {
    let cwd = std::env::current_dir().unwrap();

    // Ignore filter with 1 ignore file "test.ignore" in the root of this project
    let mut ignore_filter = IgnoreFilter::new(
        &cwd,
        &(if INCLUDE_ROOT_IGNORE_FILE {
            vec![IgnoreFile {
                path: cwd.join("test.ignore"),
                applies_in: Some(cwd.clone()),
                applies_to: None,
            }]
        } else {
            vec![]
        }),
    )
    .await
    .unwrap();

    // Manually add some globs in addition to the main ignore file
    ignore_filter
        .add_globs(
            &["files/glob"],
            Some(
                // BUG?: We need to pass `/` here in order for these globs to actually do anything
                &(if USE_PREFIXED_PATH_AS_APPLIES_IN_PARAMETER {
                    PathBuf::from(prefix(&cwd))
                } else {
                    cwd.clone()
                }),
            ),
        )
        .unwrap();

    // Test - "Glob" file
    // This file should be ignored by the glob pattern
    test(
        &ignore_filter,
        cwd.join("files/glob/i_should_be_ignored_by_globs.txt"),
        true,
    );

    // Test - "Ignore" file
    // This file should be ignored by `test.ignore`
    test(
        &ignore_filter,
        cwd.join("files/ignore/i_should_be_ignored_by_ignore_files.txt"),
        true,
    );

    // Test - Regular file
    // This file should NOT be ignored
    test(
        &ignore_filter,
        cwd.join("files/i_should_not_be_ignored_by_anything.txt"),
        false,
    );
}

fn test(ignore_filter: &IgnoreFilter, path: PathBuf, should_be_ignored: bool) {
    let match_result = ignore_filter.match_path(&path, path.is_dir());
    let is_ignored = match_result.is_ignore();

    println!("{:?} is ignored: {:?}", path, is_ignored);

    assert_eq!(
        should_be_ignored, is_ignored,
        "{:?} should be ignored. match_result: {:?}",
        &path, match_result
    );
}

// From watchexec source:
// https://github.com/watchexec/watchexec/blob/c0b01a43a39bfbc5e4aa5a19d5791326d30cb57e/crates/ignore-files/src/filter.rs#L410
fn prefix<T: AsRef<Path>>(path: T) -> String {
    let path = path.as_ref();

    let Some(prefix) = path.components().next() else {
        return "/".into();
    };

    match prefix {
        std::path::Component::Prefix(prefix_component) => {
            prefix_component.as_os_str().to_str().unwrap_or("/").into()
        }
        _ => "/".into(),
    }
}
