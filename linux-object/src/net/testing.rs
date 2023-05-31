
use byteorder::{ByteOrder, NetworkEndian};
use log::debug;

use alloc::vec;
use alloc::sync::Arc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::str::FromStr;
use core::fmt::Write;

use hashbrown::HashMap;
use kernel_hal::{NetDriver, Thread, yield_now, timer_now};
use kernel_hal::drivers::NET_DRIVERS;

use smoltcp::iface::{InterfaceBuilder, NeighborCache, Routes};
use smoltcp::phy::Device;
use smoltcp::socket::*;
use smoltcp::wire::{
    EthernetAddress, Icmpv4Packet, Icmpv4Repr, Icmpv6Packet, Icmpv6Repr, IpAddress, IpCidr,
    Ipv4Address, Ipv6Address,
};
use smoltcp::{
    phy::Medium,
    time::{Duration, Instant},
};

macro_rules! send_icmp_ping {
    ( $repr_type:ident, $packet_type:ident, $ident:expr, $seq_no:expr,
      $echo_payload:expr, $socket:expr, $remote_addr:expr ) => {{
        let icmp_repr = $repr_type::EchoRequest {
            ident: $ident,
            seq_no: $seq_no,
            data: &$echo_payload,
        };

        let icmp_payload = $socket.send(icmp_repr.buffer_len(), $remote_addr).unwrap();

        let icmp_packet = $packet_type::new_unchecked(icmp_payload);
        (icmp_repr, icmp_packet)
    }};
}

macro_rules! get_icmp_pong {
    ( $repr_type:ident, $repr:expr, $payload:expr, $waiting_queue:expr, $remote_addr:expr,
      $timestamp:expr, $received:expr ) => {{
        if let $repr_type::EchoReply { seq_no, data, .. } = $repr {
            if let Some(_) = $waiting_queue.get(&seq_no) {
                let packet_timestamp_ms = NetworkEndian::read_i64(data);
                info!(
                    "{} bytes from {}: icmp_seq={}, time={}ms",
                    data.len(),
                    $remote_addr,
                    seq_no,
                    $timestamp.total_millis() - packet_timestamp_ms
                );
                $waiting_queue.remove(&seq_no);
                $received += 1;
            }
        }
    }};
}

async fn ping_main() {

    let net_devs = NET_DRIVERS.read().len();
    if net_devs < 1 {
        loop {
            #[cfg(target_arch = "riscv64")]
            kernel_hal_bare::interrupt::wait_for_interrupt();

            warn!("NO NET DRIVERS !");
            yield_now().await;
        }
    }

    //use kernel_hal_bare::drivers::net::rtl8x::RTL8xInterface as DriverInterface;
    use kernel_hal_bare::drivers::net::e1000::E1000Interface as DriverInterface;

    let interface = (NET_DRIVERS.write()[0]).as_any().downcast_ref::<DriverInterface>().unwrap().clone();

    let ifname = interface.get_ifname();
    let ethernet_addr = interface.get_mac();
    debug!("NET_DRIVERS 1/{} read OK!\n{} MAC: {:x?}", net_devs, ifname, ethernet_addr);

    let mut iface = interface.iface.lock();
    /*
    let iface_arc = Arc::clone(&interface.iface);
    let mut iface = iface_arc.lock();
    */

    let device = interface.driver;
    let device_caps = device.capabilities();

    debug!("IP address: {}, Default gateway: {:?}", iface.ipv4_address().unwrap(), iface.routes());

    let taddr = "192.168.0.62";
    //let taddr = "10.0.2.2";
    let address = IpAddress::from_str(taddr).expect("invalid address format");
    let count = 3000;
    let interval = Duration::from_secs(1);
    let timeout = Duration::from_secs(5);

    let remote_addr = address;

    let icmp_rx_buffer = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
    let icmp_tx_buffer = IcmpSocketBuffer::new(vec![IcmpPacketMetadata::EMPTY], vec![0; 256]);
    let icmp_socket = IcmpSocket::new(icmp_rx_buffer, icmp_tx_buffer);

    let udp_rx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 64]);
    let udp_tx_buffer = UdpSocketBuffer::new(vec![UdpPacketMetadata::EMPTY], vec![0; 128]);
    let udp_socket = UdpSocket::new(udp_rx_buffer, udp_tx_buffer);
    let tcp_rx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_tx_buffer = TcpSocketBuffer::new(vec![0; 1024]);
    let tcp_socket = TcpSocket::new(tcp_rx_buffer, tcp_tx_buffer);

    /*
    let neighbor_cache = NeighborCache::new(BTreeMap::new());

    let ethernet_addr = EthernetAddress([0x02, 0x00, 0x00, 0x00, 0x00, 0x02]);
    let src_ipv6 = IpAddress::v6(0xfdaa, 0, 0, 0, 0, 0, 0, 1);
    let ip_addrs = [
        IpCidr::new(IpAddress::v4(192, 168, 0, 69), 24),
        IpCidr::new(src_ipv6, 64),
        IpCidr::new(IpAddress::v6(0xfe80, 0, 0, 0, 0, 0, 0, 1), 64),
    ];
    let default_v4_gw = Ipv4Address::new(192, 168, 0, 1);
    let default_v6_gw = Ipv6Address::new(0xfe80, 0, 0, 0, 0, 0, 0, 0x100);
    let mut routes_storage = [None; 2];
    let mut routes = Routes::new(&mut routes_storage[..]);
    routes.add_default_ipv4_route(default_v4_gw).unwrap();
    routes.add_default_ipv6_route(default_v6_gw).unwrap();

    let medium = device.capabilities().medium;
    let mut builder = InterfaceBuilder::new(device)
        .ip_addrs(ip_addrs)
        .routes(routes);
    if medium == Medium::Ethernet {
        builder = builder
            .ethernet_addr(ethernet_addr)
            .neighbor_cache(neighbor_cache);
    }
    let mut iface = builder.finalize();
    */


    let mut sockets = SocketSet::new(vec![]);
    let icmp_handle = sockets.add(icmp_socket);

    let udp_handle = sockets.add(udp_socket);
    let tcp_handle = sockets.add(tcp_socket);

    let mut send_at = Instant::from_millis(0);
    let mut seq_no = 0;
    let mut received = 0;
    let mut echo_payload = [0xffu8; 40];
    let mut waiting_queue = HashMap::new();
    let ident = 0x22b;

    loop {
        let timestamp = Instant::from_millis(timer_now().as_millis() as i64);
        match iface.poll(&mut sockets, timestamp) {
            Ok(_) => {}
            Err(e) => {
                debug!("poll error: {}", e);
            }
        }


            // UDP testing
            {
                let mut socket = sockets.get::<UdpSocket>(udp_handle);
                if !socket.is_open() {
                    socket.bind(6969).unwrap();
                    debug!("UDP test port: 6969");
                }

                let client = match socket.recv() {
                    Ok((data, endpoint)) => {
                        info!(
                            "udp:6969 recv data: {:?} from {}",
                            alloc::str::from_utf8(data).unwrap(),
                            endpoint
                        );
                        Some(endpoint)
                    }
                    Err(_) => None,
                };
                if let Some(endpoint) = client {
                    info!("UDP port 6969 recv");
                    let data = b"Greeting from zCore\n";
                    socket.send_slice(data, endpoint).unwrap();
                }
            }

            // TCP testing
            {
                let mut socket = sockets.get::<TcpSocket>(tcp_handle);
                if !socket.is_open() {
                    socket.listen(80).unwrap();
                    debug!("TCP listen port: 80");
                }

                if socket.can_send() {
                    info!("TCP port 80 recv");
                    write!(socket, "HTTP/1.1 200 OK\r\nServer: zCore\r\nContent-Length: 13\r\nContent-Type: text/html\r\nConnection: Closed\r\n\r\nHello! zCore \r\n").unwrap();
                    socket.close();
                }
            }

        #[cfg(target_arch = "riscv64")]
        kernel_hal_bare::interrupt::wait_for_interrupt();
        yield_now().await;

        {
            let timestamp = Instant::from_millis(timer_now().as_millis() as i64);
            let mut socket = sockets.get::<IcmpSocket>(icmp_handle);
            if !socket.is_open() {
                socket.bind(IcmpEndpoint::Ident(ident)).unwrap();
                send_at = timestamp;
            }

            if socket.can_send() && seq_no < count as u16 && send_at <= timestamp {
                NetworkEndian::write_i64(&mut echo_payload, timestamp.total_millis());

                match remote_addr {
                    IpAddress::Ipv4(_) => {
                        let (icmp_repr, mut icmp_packet) = send_icmp_ping!(
                            Icmpv4Repr,
                            Icmpv4Packet,
                            ident,
                            seq_no,
                            echo_payload,
                            socket,
                            remote_addr
                        );
                        icmp_repr.emit(&mut icmp_packet, &device_caps.checksum);
                    }
                    /*
                    IpAddress::Ipv6(_) => {
                        let (icmp_repr, mut icmp_packet) = send_icmp_ping!(
                            Icmpv6Repr,
                            Icmpv6Packet,
                            ident,
                            seq_no,
                            echo_payload,
                            socket,
                            remote_addr
                        );
                        icmp_repr.emit(
                            &src_ipv6,
                            &remote_addr,
                            &mut icmp_packet,
                            &device_caps.checksum,
                        );
                    }
                    */
                    _ => unimplemented!(),
                }

                waiting_queue.insert(seq_no, timestamp);
                seq_no += 1;
                send_at += interval;
            }

        #[cfg(target_arch = "riscv64")]
        kernel_hal_bare::interrupt::wait_for_interrupt();
        yield_now().await;

            if socket.can_recv() {
                let (payload, _) = socket.recv().unwrap();

                match remote_addr {
                    IpAddress::Ipv4(_) => {
                        let icmp_packet = Icmpv4Packet::new_checked(&payload).unwrap();
                        let icmp_repr =
                            Icmpv4Repr::parse(&icmp_packet, &device_caps.checksum).unwrap();
                        get_icmp_pong!(
                            Icmpv4Repr,
                            icmp_repr,
                            payload,
                            waiting_queue,
                            remote_addr,
                            timestamp,
                            received
                        );
                    }
                    /*
                    IpAddress::Ipv6(_) => {
                        let icmp_packet = Icmpv6Packet::new_checked(&payload).unwrap();
                        let icmp_repr = Icmpv6Repr::parse(
                            &remote_addr,
                            &src_ipv6,
                            &icmp_packet,
                            &device_caps.checksum,
                        )
                        .unwrap();
                        get_icmp_pong!(
                            Icmpv6Repr,
                            icmp_repr,
                            payload,
                            waiting_queue,
                            remote_addr,
                            timestamp,
                            received
                        );
                    }
                    */
                    _ => unimplemented!(),
                }
            }

            waiting_queue.retain(|seq, from| {
                if timestamp - *from < timeout {
                    true
                } else {
                    info!("From {} icmp_seq={} timeout", remote_addr, seq);
                    false
                }
            });

            if seq_no == count as u16 && waiting_queue.is_empty() {
                warn!("break ping loop");
                break;
            }
        }

        /*
        let timestamp = Instant::now();
        match iface.poll_at(&sockets, timestamp) {
            Some(poll_at) if timestamp < poll_at => {
                let resume_at = cmp::min(poll_at, send_at);
                phy_wait(fd, Some(resume_at - timestamp)).expect("wait error");
            }
            Some(_) => (),
            None => {
                phy_wait(fd, Some(send_at - timestamp)).expect("wait error");
            }
        }
        */

        #[cfg(target_arch = "riscv64")]
        kernel_hal_bare::interrupt::wait_for_interrupt();
        yield_now().await;
    }

    info!("--- {} ping statistics ---", remote_addr);
    info!(
        "{} packets transmitted, {} received, {:.0}% packet loss",
        seq_no,
        received,
        100.0 * (seq_no - received) as f64 / seq_no as f64
    );
}

pub fn net_ping_thread() {
    let pin_future = Box::pin(ping_main());
    let vmtoken = kernel_hal::current_page_table();
    Thread::spawn(pin_future, vmtoken);
}
