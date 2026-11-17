// Integration test for SigmaOS-tools-codex-cli
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use tools_codex_cli as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
