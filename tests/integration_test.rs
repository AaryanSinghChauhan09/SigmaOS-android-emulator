//! Integration tests for SigmaOS-android-emulator2.

use sigmaos_android_emulator2_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
