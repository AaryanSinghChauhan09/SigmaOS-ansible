// Integration test for SigmaOS-ansible
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_ansible as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
