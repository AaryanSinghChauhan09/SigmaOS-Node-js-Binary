//! Integration tests for SigmaOS-Node-js-Binary.

use sigmaos_node_js_binary_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
