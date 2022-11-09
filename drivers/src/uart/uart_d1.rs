use core::convert::TryInto;
use core::ops::{BitAnd, BitOr, Not};

use bitflags::bitflags;
//use spin::Mutex;
use lock::Mutex;

use crate::io::{Io, Mmio, ReadOnly};
use crate::scheme::{impl_event_scheme, Scheme, UartScheme};
use crate::utils::EventListener;
use crate::DeviceResult;

bitflags! {
    /// Interrupt enable flags
    struct IntEnFlags: u32 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    struct LineStsFlags: u32 {
        const INPUT_FULL = 1;
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5;
        // 6 and 7 unknown
    }
}

#[repr(C)]
struct Uart16550Inner<T: Io> {
    /// Data register, read to receive, write to send
    data: T,
    /// Interrupt enable
    int_en: T,
    /// FIFO control
    fifo_ctrl: T,
    /// Line control
    line_ctrl: T,
    /// Modem control
    modem_ctrl: T,
    /// Line status
    line_sts: ReadOnly<T>,
    /// Modem status
    modem_sts: ReadOnly<T>,
    /// scratch
    scratch: T,
    /// padding
    padding1: [T; 23],
    /// uart status, 0x7C
    usr: T,
    /// padding
    padding2: [T; 9],
    /// Halt TX Register, 0xA4
    halt: T,
}

impl<T: Io> Uart16550Inner<T>
where
    T::Value: From<u32> + TryInto<u32>,
{
    // TODO, Fix bug
    #[allow(dead_code)]
    fn init(&mut self) {
        // Disable interrupts
        self.int_en.write(0x00.into());

        self.fifo_ctrl.write(0xb1.into());

        // hold tx so that uart will update lcr and baud in the gap of rx
        {
        // Halt TX enabled, Enable change when busy
        self.halt.write(0x03.into());

        // Enable DLAB
        self.line_ctrl.write((0x80 | 0x03).into());

        let baud_divisor = 13;
        self.data.write((baud_divisor & 0xff).into());
        self.int_en.write((baud_divisor >> 8).into());

        // HALT_TX, CHCFG_AT_BUSY, CHANGE_UPDATE
        self.halt.write(0x07.into());
        // Check CHANGE_UPDATE in halt reg
        while (self.halt.read() & 0x04.into()).try_into().unwrap_or(0) != 0 {}

        // Disable DLAB and set data word length to 8 bits
        self.line_ctrl.write(0x03.into());
        // change config when busy
        self.halt.write(0x02.into());
        }

        // clear rxfifo after set baud
        self.fifo_ctrl.write((0xb1 | 0x02).into());
        self.modem_ctrl.write(0x03.into());

        // Enable interrupts
        self.int_en.write(0x1.into());
    }

    #[allow(dead_code)]
    fn init_uart(&mut self) {
        // Waiting TX to be empty. OR setting halt reg
        while (self.line_sts.read() & 0x40.into()).try_into().unwrap_or(0) == 0 {}

        // Disable interrupts
        self.int_en.write(0x00.into());

        // Mark data terminal ready, signal request to send
        // and enable auxilliary output #2 (used as interrupt line for CPU)
        self.modem_ctrl.write(0x03.into());
        // Enable FIFO, clear TX/RX queues and
        // set interrupt watermark at 14 bytes
        self.fifo_ctrl.write(0x07.into());

        {
        // Enable DLAB
        self.line_ctrl.write((0x80 | 0x03).into());

        // NS16550, Set maximum speed to 115200 bps by configuring DLL and DLM
        // clock: 24000000, baudrate: 115200, baud_divisor: 13
        // clock: 24000000, baudrate: 57600, baud_divisor: 26
        // ( clock + (baudrate * 16)/2 ) / (baudrate * 16) = baud_divisor = ( clock / (baudrate * 16) ) + 0.5
        // 
        let baud_divisor = 13;
        self.data.write((baud_divisor & 0xff).into());
        self.int_en.write((baud_divisor >> 8).into());

        // Disable DLAB and set data word length to 8 bits
        self.line_ctrl.write(0x03.into());
        }

        // Enable interrupts
        self.int_en.write(0x1.into());
    }

    #[allow(dead_code)]
    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(
            (self.line_sts.read() & 0xFF.into()).try_into().unwrap_or(0),
        )
    }

    fn try_recv(&mut self) -> DeviceResult<Option<u8>> {
        // while read(lsr) & 1 == 0 {}
        if (self.usr.read() & 0x08.into()).try_into().unwrap_or(0) != 0 {
            Ok(Some(
                (self.data.read() & 0xFF.into()).try_into().unwrap_or(0) as u8
                // T::Value impl From<u32>, 8.into() => T::Value ;   
                // T::Value impl TryInto<u32>, T::Value.into() => u32;
            ))
        } else {
            Ok(None)
        }
    }

    fn send(&mut self, ch: u8) -> DeviceResult {
        // while read(lsr) & 0x20 == 0 {}
        while (self.usr.read() & 0x02.into()).try_into().unwrap_or(0) == 0 {}
        // impl trait `From<u8>` for T::Value
        self.data.write((ch as u32).into());
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> DeviceResult {
        for b in s.bytes() {
            match b {
                b'\n' => {
                    self.send(b'\r')?;
                    self.send(b'\n')?;
                }
                _ => {
                    self.send(b)?;
                }
            }
        }
        Ok(())
    }
}

pub struct Uart16550D1<V: 'static>
where
    V: Copy + BitAnd<Output = V> + BitOr<Output = V> + Not<Output = V>,
{
    inner: Mutex<&'static mut Uart16550Inner<Mmio<V>>>,
    listener: EventListener,
}

impl_event_scheme!(Uart16550D1<V>
where
    V: Copy
        + BitAnd<Output = V>
        + BitOr<Output = V>
        + Not<Output = V>
        + From<u32>
        + TryInto<u32>
        + Send
);

impl<V> Scheme for Uart16550D1<V>
where
    V: Copy + BitAnd<Output = V> + BitOr<Output = V> + Not<Output = V> + Send,
{
    fn name(&self) -> &str {
        "d1-uart16550-mmio"
    }

    fn handle_irq(&self, irq_num: usize) {
        debug!("uart d1 handle_irq: {:?}", irq_num);
        self.listener.trigger(());
    }
}

impl<V> UartScheme for Uart16550D1<V>
where
    V: Copy
        + BitAnd<Output = V>
        + BitOr<Output = V>
        + Not<Output = V>
        + From<u32>
        + TryInto<u32>
        + Send,
{
    fn try_recv(&self) -> DeviceResult<Option<u8>> {
        self.inner.lock().try_recv()
    }

    fn send(&self, ch: u8) -> DeviceResult {
        self.inner.lock().send(ch)
    }

    fn write_str(&self, s: &str) -> DeviceResult {
        self.inner.lock().write_str(s)
    }
}

impl<V> Uart16550D1<V>
where
    V: Copy
        + BitAnd<Output = V>
        + BitOr<Output = V>
        + Not<Output = V>
        + From<u32>
        + TryInto<u32>
        + Send,
{
    unsafe fn new_common(base: usize) -> Self {
        let uart: &mut Uart16550Inner<Mmio<V>> = Mmio::<V>::from_base_as(base);
        uart.init();
        //uart.init_uart();
        Self {
            inner: Mutex::new(uart),
            listener: EventListener::new(),
        }
    }
}

/*
impl Uart16550D1<u8> {
    /// # Safety
    ///
    /// This function is unsafe because `base_addr` may be an arbitrary address.
    pub unsafe fn new(base: usize) -> Self {
        Self::new_common(base)
    }
}
*/

impl Uart16550D1<u32> {
    /// # Safety
    ///
    /// This function is unsafe because `base_addr` may be an arbitrary address.
    pub unsafe fn new(base: usize) -> Self {
        Self::new_common(base)
    }
}
