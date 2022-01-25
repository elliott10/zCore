use nezha_sdc::MmcHost as SdMmc;

use spin::Mutex;
use alloc::sync::Arc;

use crate::scheme::{BlockScheme, Scheme};
use crate::DeviceResult;

pub struct SdMmcBlk {
    inner: Arc<Mutex<SdMmc>>,
}

impl SdMmcBlk {
    pub fn new<F: Fn(usize, usize) -> Option<usize>>(mapper: F) 
    -> DeviceResult<Self> 
    {
        Ok(Self {
            inner: nezha_sdc::init(mapper),
        })
    }
}

impl Scheme for SdMmcBlk {
    fn name(&self) -> &str {
        "sdmmc-blk"
    }

    fn handle_irq(&self, _irq_num: usize) {
        // self.inner.lock().ack_interrupt();
    }
}

impl BlockScheme for SdMmcBlk {
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> DeviceResult {
        info!("sdcard try to read block {}", block_id);
        self.inner.lock().read_block(block_id, buf);
        Ok(())
    }

    fn write_block(&self, block_id: usize, buf: &[u8]) -> DeviceResult {
        info!("sdcard try to write block {}", block_id);
        self.inner.lock().write_block(block_id, buf);
        Ok(())
    }

    fn flush(&self) -> DeviceResult {
        Ok(())
    }
}
