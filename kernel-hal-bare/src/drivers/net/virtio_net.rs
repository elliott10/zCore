use alloc::format;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::collections::BTreeMap;

use smoltcp::phy::{self, DeviceCapabilities};
use smoltcp::iface::{InterfaceBuilder, NeighborCache};
use smoltcp::time::Instant;
use smoltcp::wire::{EthernetAddress, IpAddress, Ipv4Address, IpCidr};
use smoltcp::Result;
use virtio_drivers::{VirtIOHeader, VirtIONet};
use device_tree::Node;

use super::super::{IRQ_MANAGER, device_tree::DEVICE_TREE_INTC};
use kernel_hal::drivers::{Driver, DeviceType, NetDriver, DRIVERS, NET_DRIVERS};
//use crate::{drivers::BlockDriver, sync::SpinNoIrqLock as Mutex};
use spin::Mutex;

#[derive(Clone)]
pub struct VirtIONetDriver(Arc<Mutex<VirtIONet<'static>>>);

impl NetDriver for VirtIONetDriver {
    fn get_mac(&self) -> EthernetAddress {
        EthernetAddress(self.0.lock().mac())
    }

    fn get_ifname(&self) -> String {
        format!("virtio{:?}", self.0.lock().mac())
    }

    fn ipv4_address(&self) -> Option<Ipv4Address> {
        unimplemented!()
    }

    fn poll(&self) {
        unimplemented!()
    }
}

impl Driver for VirtIONetDriver {
    fn try_handle_interrupt(&self, irq: Option<usize>) -> bool {
        info!("VirtIONetDriver got interrupt {:?}", irq);
        //iface.poll()时中断内发生死锁,暂关闭该中断处理
        self.0.lock().ack_interrupt()
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Net
    }

    fn get_id(&self) -> String {
        format!("virtio_net")
    }

    fn as_net(&self) -> Option<&dyn NetDriver> {
        Some(self)
    }

    /*
    fn as_block(&self) -> Option<&dyn BlockDriver> {
        None
    }
    */
}

impl phy::Device<'_> for VirtIONetDriver {
    type RxToken = VirtIONetDriver;
    type TxToken = VirtIONetDriver;

    fn receive(&mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        /*
        let net = self.0.lock();
        let r = net.can_recv();
        */

        //初始时，由于没有添加recv_queue和写queue_notify
        //故can_recv()会一直返回false
        //这里的判断等待包的过程转移到consume()的driver.recv()中去做吧
        //当然最好的方式应该在此调用driver.recv()
        if true {
            Some((self.clone(), self.clone()))
        } else {
            None
        }
    }

    fn transmit(&mut self) -> Option<Self::TxToken> {
        let net = self.0.lock();
        if net.can_send() {
            info!("phy::Device transmit");
            Some(self.clone())
        } else {
            None
        }
    }

    fn capabilities(&self) -> DeviceCapabilities {
        //info!("phy::Device capabilities()");
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1536;
        caps.max_burst_size = Some(1);
        caps
    }
}

impl phy::RxToken for VirtIONetDriver {
    fn consume<R, F>(self, _timestamp: Instant, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        info!("RxToken recv consume()");
        let mut buffer = [0u8; 2000];
        let mut len = buffer.len();
        { //若无括号会与TxToken consume中的lock()发生死锁
            let mut driver = self.0.lock();
            //需要添加recv_queue和写queue_notify，才能触发virtioNet网卡中断一次?
            //这里有等待总能收到包，TODO: fix me
            len = driver.recv(&mut buffer).expect("failed to recv packet");
        }

        f(&mut buffer[..len])
    }
}

impl phy::TxToken for VirtIONetDriver {
    fn consume<R, F>(self, _timestamp: Instant, len: usize, f: F) -> Result<R>
    where
        F: FnOnce(&mut [u8]) -> Result<R>,
    {
        info!("TxToken send consume()");
        let mut buffer = [0u8; 2000];
        let result = f(&mut buffer[..len]);
        //发生死锁
        let mut driver = self.0.lock();
        driver.send(&buffer[..len]).expect("failed to send packet");
        result
    }
}

pub fn init(node: &Node, header: &'static mut VirtIOHeader) {
    debug!("virtio net init");
    let net = VirtIONet::new(header).expect("failed to create net driver");
    //let mac = net.mac();

    let device = VirtIONetDriver(Arc::new(Mutex::new(net)));

    /* Todo like e1000
    // let device = Loopback::new(Medium::Ethernet);
    let hw_addr = EthernetAddress::from_bytes(&mac);
    let neighbor_cache = NeighborCache::new(BTreeMap::new());
    let ip_addrs = [IpCidr::new(IpAddress::v4(10, 0, 2, 15), 24)];
    let iface = InterfaceBuilder::new(device.clone())
        .ethernet_addr(hw_addr)
        .neighbor_cache(neighbor_cache)
        .ip_addrs(ip_addrs)
        .finalize();
    */
    //let driver = Arc::new(iface);

    let driver = Arc::new(device);

    /*
    // Enable VirtIO Network interrupt
    let mut found = false;
    let irq_opt = node.prop_u32("interrupts").ok().map(|irq| irq as usize);

    if let Ok(intc) = node.prop_u32("interrupt-parent") {
        if let Some(irq) = irq_opt {
            if let Some(manager) = DEVICE_TREE_INTC.write().get_mut(&intc) {
                manager.register_local_irq(irq, driver.clone());
                info!("Registed virtio net irq {} to INTC", irq);
                found = true;
            }
        }
    }
    if !found {
        info!("Registed virtio net driver to ROOT");
        IRQ_MANAGER.write().register_opt(irq_opt, driver.clone());
    }

    {
        let mut buffer = [0u8; 2000];
        let mut dri = driver.0.lock();
        //触发virtioNet网卡中断一次。 不过现在中断hanlde会死锁
        let len = dri.recv(&mut buffer).expect("failed to recv packet");
    }
    */

    DRIVERS.write().push(driver.clone());
    NET_DRIVERS.write().push(driver);
}
