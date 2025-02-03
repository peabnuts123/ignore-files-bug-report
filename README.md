# Bug: `add_globs()` only seems to work if there's already an ignore file at `applies_in`

This is a minimal reproduction for a [bug report](https://github.com/watchexec/watchexec/issues/907).

## Issue

Calling `IgnoreFilter::add_globs()` does not seem to have any effect unless there is already an IgnoreFile registered at `applies_in`.

## Reproduction

This project has a couple of simple tests to assert whether certain files are ignored or not.

 - `files/glob/i_should_be_ignored_by_globs.txt`
   - Should be ignored by glob pattern passed to `add_globs()`
 - `files/ignore/i_should_be_ignored_by_ignore_files.txt`
   - Should be ignored by glob pattern specified in `test.ignore`
 - `files/i_should_not_be_ignored_by_anything.txt`
   - Should NOT be ignored

### Config

In `main.rs` are 2 config flags:

 - `INCLUDE_ROOT_IGNORE_FILE`
   - This includes an ignore file at the root of the IgnoreFilter: `test.ignore`
   - Setting this to true will mean `add_globs()` works as-expected due to the presence of an ignore file at the location of `applies_in`
   - Settings this to false will mean the IgnoreFilter is constructed without any ignore files, which will cause `add_globs()` to fail
 - `USE_PREFIXED_PATH_AS_APPLIES_IN_PARAMETER`
   - This applies a workaround to `add_globs()` to pass `/` (or `C:` etc on Windows) in as the `applies_in` parameter
   - Setting this to true will resolve the issue even if `INCLUDE_ROOT_IGNORE_FILE` is false

It's worth mentioning that not all tests necessarily pass all the time. For example, the test for `i_should_be_ignored_by_ignore_files.txt` will not pass unless `INCLUDE_ROOT_IGNORE_FILE` is true - since the ignore file won't be included. The tests are just clarifying what behaviour is occurring.

For simplicity, below is a table of the exhibited behaviours based on the 2 config values:

| INCLUDE_ROOT_IGNORE_FILE | USE_PREFIXED_PATH_AS_APPLIES_IN_PARAMETER | Outcome |
| ------------------------ | ----------------------------------------- | ------- |
| `false`                  | `false`                                   | BUG(?): `add_globs` will have no effect. |
| `false`                  | `true`                                    | `add_globs` works as intended since `applies_in` is `/`. |
| `true`                   | `false`                                   | `add_globs` works as intended since an ignore file exists at `applies_in`. |
| `true`                   | `true`                                    | `add_globs` works as intended due to both reasons listed above. |
