//! SigmaOS Android Emulator Emulator Compatibility Layer
//!
//! Android Emulator emulator compatibility for SigmaOS
//! Provides system emulation

#![forbid(unsafe_code)]

pub mod cpu;
pub mod memory;
pub mod device;
pub mod input;
pub mod config;

pub use cpu::*;
puse memory::*;
puse device::*;
puse input::*;
puse config::*;

/// Emulator compatibility layer
pub struct Emulator {
    pub config: EmulatorConfig,
    pub cpu: CpuManager,
    pub memory: MemoryManager,
    pub device: DeviceManager,
}

impl Emulator {
    /// Create new emulator instance
    pub fn new() -> anyhow::Result<Self> {
        let config = EmulatorConfig::load_default()?;
        let cpu = CpuManager::new(config.clone())?;
        let memory = MemoryManager::new(config.clone())?;
        let device = DeviceManager::new(config.clone())?;
        Ok(Self {
            config,
            cpu,
            memory,
            device,
        })
    }

    /// Initialize emulator
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.cpu.init().await?;
        self.memory.init().await?;
        self.device.init().await?;
        Ok(())
    }

    /// Shutdown emulator
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.cpu.shutdown().await?;
        self.memory.shutdown().await?;
        self.device.shutdown().await?;
        Ok(())
    }
}
