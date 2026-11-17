// Integration test for SigmaOS-Node-js-Binary
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use SigmaOS_Node_js_Binary as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
