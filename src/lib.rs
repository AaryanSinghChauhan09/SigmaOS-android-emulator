//! Android emulator configuration and profile manager for SigmaOS.
//!
//! Lightweight emulator profile management: AVD creation, device specs,
//! and launch arguments. Dependency-free.

use std::collections::HashMap;
use std::fmt;

/// Errors for emulator operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmulatorError {
    InvalidDeviceName(String),
    MissingProfile(String),
    InvalidCpuArch(String),
}

impl fmt::Display for EmulatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDeviceName(n) => write!(f, "invalid device name: {n}"),
            Self::MissingProfile(p) => write!(f, "missing profile: {p}"),
            Self::InvalidCpuArch(a) => write!(f, "invalid CPU architecture: {a}"),
        }
    }
}

impl std::error::Error for EmulatorError {}

/// CPU architecture options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuArch {
    X86_64,
    Arm64,
    X86,
    Arm,
}

impl CpuArch {
    pub fn from_str(s: &str) -> Result<Self, EmulatorError> {
        match s.to_lowercase().as_str() {
            "x86_64" | "x86-64" => Ok(Self::X86_64),
            "arm64" | "aarch64" => Ok(Self::Arm64),
            "x86" | "i386" => Ok(Self::X86),
            "arm" => Ok(Self::Arm),
            _ => Err(EmulatorError::InvalidCpuArch(s.to_string())),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X86_64 => "x86_64",
            Self::Arm64 => "arm64",
            Self::X86 => "x86",
            Self::Arm => "arm",
        }
    }
}

/// Device profile (AVD-like).
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceProfile {
    pub name: String,
    pub cpu: CpuArch,
    pub ram_mb: u32,
    pub disk_gb: u32,
}

impl DeviceProfile {
    pub fn new(name: impl Into<String>, cpu: CpuArch, ram_mb: u32, disk_gb: u32) -> Self {
        Self {
            name: name.into(),
            cpu,
            ram_mb,
            disk_gb,
        }
    }

    pub fn validate(&self) -> Result<(), EmulatorError> {
        if self.name.trim().is_empty() {
            return Err(EmulatorError::InvalidDeviceName(self.name.clone()));
        }
        if self.ram_mb == 0 || self.disk_gb == 0 {
            return Err(EmulatorError::InvalidDeviceName("invalid resources".into()));
        }
        Ok(())
    }
}

/// Emulator profile manager.
#[derive(Debug, Clone, Default)]
pub struct EmulatorManager {
    profiles: HashMap<String, DeviceProfile>,
}

impl EmulatorManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_profile(&mut self, profile: DeviceProfile) -> Result<(), EmulatorError> {
        profile.validate()?;
        let name = profile.name.clone();
        self.profiles.insert(name, profile);
        Ok(())
    }

    pub fn get_profile(&self, name: &str) -> Option<&DeviceProfile> {
        self.profiles.get(name)
    }

    pub fn list_profiles(&self) -> Vec<&str> {
        self.profiles.keys().map(|k| k.as_str()).collect()
    }

    pub fn remove_profile(&mut self, name: &str) -> Result<(), EmulatorError> {
        self.profiles.remove(name).ok_or_else(|| EmulatorError::MissingProfile(name.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_validation_aaa() {
        // Arrange
        let profile = DeviceProfile::new("pixel5", CpuArch::Arm64, 4096, 8);

        // Act
        let res = profile.validate();

        // Assert
        assert!(res.is_ok());
    }

    #[test]
    fn test_invalid_name_rejected_aaa() {
        // Arrange
        let profile = DeviceProfile::new("", CpuArch::Arm64, 4096, 8);

        // Act
        let res = profile.validate();

        // Assert
        assert!(res.is_err());
    }

    #[test]
    fn test_manager_add_remove_aaa() {
        // Arrange
        let mut mgr = EmulatorManager::new();
        let profile = DeviceProfile::new("pixel5", CpuArch::Arm64, 4096, 8);

        // Act
        mgr.add_profile(profile.clone()).unwrap();
        let retrieved = mgr.get_profile("pixel5");
        let name = retrieved.map(|p| p.name.clone());
        drop(retrieved);
        mgr.remove_profile("pixel5").unwrap();

        // Assert
        assert!(name.unwrap() == "pixel5");
        assert!(mgr.get_profile("pixel5").is_none());
    }
}
