pub mod e1000;
pub mod ixgbe;
pub mod virtio_net;

#[cfg(target_arch = "riscv64")]
pub mod rtl8x;

#[cfg(target_arch = "riscv64")]
pub mod realtek;
