use nezha_sdc::MmcHost as SdMmc;

use spin::Mutex;

use alloc::sync::Arc;

use core::sync::atomic::{AtomicBool, Ordering};

use crate::scheme::{BlockScheme, Scheme};
use crate::DeviceResult;

pub struct SdMmcBlk {
    init: AtomicBool,
    inner: Arc<SdMmc>,
}

impl SdMmcBlk {
    pub fn new<F: Fn(usize, usize) -> Option<usize>>(mapper: F) -> DeviceResult<Self> {
        Ok(Self {
            init: AtomicBool::new(false),
            inner: nezha_sdc::primary_init(mapper),
        })
    }

    pub fn check_and_init(&self) -> DeviceResult {
        if self.init.load(Ordering::Relaxed) == false {
            info!("init sdcard !!!");
            nezha_sdc::init(&self.inner);
            info!("init sdcard over !!!");
            self.init.store(true, Ordering::Relaxed);
        }
        Ok(())
    }
}

impl Scheme for SdMmcBlk {
    fn name(&self) -> &str {
        "sdmmc-blk"
    }

    fn handle_irq(&self, _irq_num: usize) {
        info!("drivers: handle irq");
        self.inner.handle_irq();
    }
}

impl BlockScheme for SdMmcBlk {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> DeviceResult {
        info!("sdcard try to read block {}", block_id);
        self.check_and_init()?;
        self.inner.read_block(block_id, buf);
        Ok(())
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> DeviceResult {
        info!("sdcard try to write block {}", block_id);
        self.check_and_init()?;
        self.inner.write_block(block_id, buf);
        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        self.check_and_init()?;
        Ok(())
    }
}
