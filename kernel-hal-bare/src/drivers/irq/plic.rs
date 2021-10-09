//! RISC-V plic

pub use kernel_hal::drivers::{Driver, DeviceType, DRIVERS};
use super::{super::IRQ_MANAGER, IntcDriver, IrqManager};
use crate::drivers::{
    device_tree::DEVICE_TREE_INTC, device_tree::DEVICE_TREE_REGISTRY,
};
use crate::phys_to_virt;
//use crate::{sync::SpinNoIrqLock as Mutex, util::read, util::write};
use spin::Mutex;
use core::ptr::{read_volatile, write_volatile};
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use device_tree::Node;

pub struct Plic {
    base: usize,
    manager: Mutex<IrqManager>,
}

impl Driver for Plic {
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool {
        // TODO: support more than 32 irqs
        let pending: u32 = read(self.base + 0x1000);
        if pending != 0 {
            let claim: u32 = read(self.base + 0x201004);
            let manager = self.manager.lock();
            let res = manager.try_handle_interrupt(Some(claim as usize));
            // complete
            write(self.base + 0x201004, claim);
            res
        } else {
            false
        }
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Intc
    }

    fn get_id(&self) -> String {
        format!("plic_{}", self.base)
    }
}

impl IntcDriver for Plic {
    /// Register interrupt controller local irq
    fn register_local_irq(&self, irq: usize, driver: Arc<dyn Driver>) {
        // enable irq for context 1
        write(
            self.base + 0x2080,
            read::<u32>(self.base + 0x2080) | (1 << irq),
        );
        // set priority to 7
        write(self.base + irq * 4, 7);
        let mut manager = self.manager.lock();
        manager.register_irq(irq, driver);
    }
}

pub const SupervisorExternal: usize = usize::MAX / 2 + 1 + 8;

fn init_dt(dt: &Node) {
    let addr = dt.prop_u64("reg").unwrap() as usize;
    let phandle = dt.prop_u32("phandle").unwrap();
    info!("Found riscv plic at {:#x}, {:?}", addr, dt);
    let base = phys_to_virt(addr);
    let plic = Arc::new(Plic {
        base,
        manager: Mutex::new(IrqManager::new(false)),
    });
    // set prio threshold to 0 for context 1
    write(base + 0x201000, 0);

    DRIVERS.write().push(plic.clone());
    // register under root irq manager
    IRQ_MANAGER
        .write()
        .register_irq(SupervisorExternal, plic.clone());
    // register interrupt controller
    DEVICE_TREE_INTC.write().insert(phandle, plic);
}

pub fn driver_init() {
    DEVICE_TREE_REGISTRY.write().insert("riscv,plic0", init_dt);
}

#[inline(always)]
pub fn write<T>(addr: usize, content: T) {
    let cell = (addr) as *mut T;
    unsafe {
        write_volatile(cell, content);
    }
}

#[inline(always)]
pub fn read<T>(addr: usize) -> T {
    let cell = (addr) as *const T;
    unsafe { read_volatile(cell) }
}