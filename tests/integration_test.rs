// Integration test for SigmaOS-android-emulator2
// Verifies the crate builds and links as a dependency.

use sigmaos_android_emulator2::{CpuArch, DeviceProfile, EmulatorManager};

#[test]
fn crate_links_and_runs() {
    // Arrange
    let mut mgr = EmulatorManager::new();
    let profile = DeviceProfile::new("pixel5", CpuArch::Arm64, 4096, 8);

    // Act
    mgr.add_profile(profile).unwrap();
    let retrieved = mgr.get_profile("pixel5");
    let name = retrieved.map(|p| p.name.clone());
    drop(retrieved);

    // Assert
    assert!(name.unwrap() == "pixel5");
}
