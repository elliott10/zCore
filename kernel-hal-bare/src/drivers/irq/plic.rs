//! RISC-V plic

use super::super::{read, write, IRQ_MANAGER};
use super::{IntcDriver, IrqManager};
use crate::drivers::{device_tree::DEVICE_TREE_INTC, device_tree::DEVICE_TREE_REGISTRY};
use crate::phys_to_virt;
pub use kernel_hal::drivers::{DeviceType, Driver, DRIVERS};
//use crate::{sync::SpinNoIrqLock as Mutex, util::read, util::write};
use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use device_tree::Node;
use spin::Mutex;

pub struct Plic {
    base: usize,
    manager: Mutex<IrqManager>,
}

impl Driver for Plic {
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool {
        // Supported more than 32 irqs
        /* Int id is pending
        let id = 10;
        let step = ((id / 32) * 4) as usize; //4节
        let pending: u32 = read(self.base + step + 0x1000);
        let is_pending = (pending & (1 << id%32)) != 0;
        debug!("Plic handle irq, Is {} pending: {}", id, is_pending);
        */

        let claim: u32 = read(self.base + 0x201004);
        if claim != 0 {
            //debug!("Plic handle irq: {}", claim);

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
        let step = (irq / 32) * 4;
        // enable irq for context 1
        write(
            self.base + step + 0x2080,
            read::<u32>(self.base + step + 0x2080) | (1 << irq % 32),
        );
        // set priority to 7
        write(self.base + irq * 4, 7);
        let mut manager = self.manager.lock();
        manager.register_irq(irq, driver);
    }
}

pub const SupervisorExternal: usize = usize::MAX / 2 + 1 + 8;

pub fn init_dt(dt: &Node) {
    let addr = dt.prop_u64("reg").unwrap() as usize;
    let phandle = dt.prop_u32("phandle").unwrap();
    info!("Found riscv plic at {:#x}, {:?}", addr, dt);
    let base = phys_to_virt(addr);
    let plic = Arc::new(Plic {
        base,
        manager: Mutex::new(IrqManager::new(false)),
    });

    // Clear PLIC S mode en regs: 1 ~ 1023
    let mut enaddr = 0x2080;
    while enaddr <= 0x20fc {
        write(base + enaddr, 0 as u32);
        enaddr += 4; // u32
    }
    info!("Clear plic enable regs untill {:#x}", enaddr - 4);

    // set prio threshold to 0 for context 1
    write(base + 0x201000, 0);

    DRIVERS.write().push(plic.clone());
    // register under root irq manager
    IRQ_MANAGER
        .write()
        .register_irq(SupervisorExternal, plic.clone());
    // register interrupt controller. phandle: 3
    DEVICE_TREE_INTC.write().insert(phandle, plic);
}

pub fn driver_init() {
    DEVICE_TREE_REGISTRY.write().insert("riscv,plic0", init_dt);
}
