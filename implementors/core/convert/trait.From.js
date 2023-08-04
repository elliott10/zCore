(function() {var implementors = {};
implementors["kernel_hal"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"kernel_hal/enum.CachePolicy.html\" title=\"enum kernel_hal::CachePolicy\">CachePolicy</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"enum\" href=\"kernel_hal/enum.IpiReason.html\" title=\"enum kernel_hal::IpiReason\">IpiReason</a>","synthetic":false,"types":["kernel_hal::common::ipi::IpiReason"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"kernel_hal/enum.IpiReason.html\" title=\"enum kernel_hal::IpiReason\">IpiReason</a>&gt; for <a class=\"type\" href=\"kernel_hal/type.IpiEntry.html\" title=\"type kernel_hal::IpiEntry\">IpiEntry</a>","synthetic":false,"types":["kernel_hal::common::ipi::IpiEntry"]},{"text":"impl&lt;T, P:&nbsp;<a class=\"trait\" href=\"kernel_hal/user/trait.Policy.html\" title=\"trait kernel_hal::user::Policy\">Policy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"kernel_hal/user/struct.UserPtr.html\" title=\"struct kernel_hal::user::UserPtr\">UserPtr</a>&lt;T, P&gt;","synthetic":false,"types":["kernel_hal::common::user::UserPtr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"kernel_hal/drivers/prelude/enum.DeviceError.html\" title=\"enum kernel_hal::drivers::prelude::DeviceError\">DeviceError</a>&gt; for <a class=\"struct\" href=\"kernel_hal/struct.HalError.html\" title=\"struct kernel_hal::HalError\">HalError</a>","synthetic":false,"types":["kernel_hal::common::defs::HalError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"kernel_hal/struct.MMUFlags.html\" title=\"struct kernel_hal::MMUFlags\">MMUFlags</a>&gt; for ProtFlags","synthetic":false,"types":["nix::sys::mman::ProtFlags"]}];
implementors["linux_object"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zircon_object/error/enum.ZxError.html\" title=\"enum zircon_object::error::ZxError\">ZxError</a>&gt; for <a class=\"enum\" href=\"linux_object/error/enum.LxError.html\" title=\"enum linux_object::error::LxError\">LxError</a>","synthetic":false,"types":["linux_object::error::LxError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/fs/vfs/enum.FsError.html\" title=\"enum linux_object::fs::vfs::FsError\">FsError</a>&gt; for <a class=\"enum\" href=\"linux_object/error/enum.LxError.html\" title=\"enum linux_object::error::LxError\">LxError</a>","synthetic":false,"types":["linux_object::error::LxError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"kernel_hal/common/user/enum.Error.html\" title=\"enum kernel_hal::common::user::Error\">Error</a>&gt; for <a class=\"enum\" href=\"linux_object/error/enum.LxError.html\" title=\"enum linux_object::error::LxError\">LxError</a>","synthetic":false,"types":["linux_object::error::LxError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"struct\" href=\"linux_object/fs/struct.FileDesc.html\" title=\"struct linux_object::fs::FileDesc\">FileDesc</a>","synthetic":false,"types":["linux_object::fs::FileDesc"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>&gt; for <a class=\"struct\" href=\"linux_object/fs/struct.FileDesc.html\" title=\"struct linux_object::fs::FileDesc\">FileDesc</a>","synthetic":false,"types":["linux_object::fs::FileDesc"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"linux_object/fs/struct.FileDesc.html\" title=\"struct linux_object::fs::FileDesc\">FileDesc</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"linux_object/fs/struct.FileDesc.html\" title=\"struct linux_object::fs::FileDesc\">FileDesc</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/socket_address/enum.Endpoint.html\" title=\"enum linux_object::net::socket_address::Endpoint\">Endpoint</a>&gt; for <a class=\"union\" href=\"linux_object/net/socket_address/union.SockAddr.html\" title=\"union linux_object::net::socket_address::SockAddr\">SockAddr</a>","synthetic":false,"types":["linux_object::net::socket_address::SockAddr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for <a class=\"enum\" href=\"linux_object/net/socket_address/enum.AddressFamily.html\" title=\"enum linux_object::net::socket_address::AddressFamily\">AddressFamily</a>","synthetic":false,"types":["linux_object::net::socket_address::AddressFamily"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/socket_address/enum.AddressFamily.html\" title=\"enum linux_object::net::socket_address::AddressFamily\">AddressFamily</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for <a class=\"enum\" href=\"linux_object/net/netlink/enum.NetlinkMessageType.html\" title=\"enum linux_object::net::netlink::NetlinkMessageType\">NetlinkMessageType</a>","synthetic":false,"types":["linux_object::net::netlink::NetlinkMessageType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/netlink/enum.NetlinkMessageType.html\" title=\"enum linux_object::net::netlink::NetlinkMessageType\">NetlinkMessageType</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>&gt; for <a class=\"enum\" href=\"linux_object/net/netlink/enum.RouteAttrTypes.html\" title=\"enum linux_object::net::netlink::RouteAttrTypes\">RouteAttrTypes</a>","synthetic":false,"types":["linux_object::net::netlink::RouteAttrTypes"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/netlink/enum.RouteAttrTypes.html\" title=\"enum linux_object::net::netlink::RouteAttrTypes\">RouteAttrTypes</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.Domain.html\" title=\"enum linux_object::net::Domain\">Domain</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.SocketType.html\" title=\"enum linux_object::net::SocketType\">SocketType</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.Protocol.html\" title=\"enum linux_object::net::Protocol\">Protocol</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.Level.html\" title=\"enum linux_object::net::Level\">Level</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.SolOptname.html\" title=\"enum linux_object::net::SolOptname\">SolOptname</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.TcpOptname.html\" title=\"enum linux_object::net::TcpOptname\">TcpOptname</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/net/enum.IpOptname.html\" title=\"enum linux_object::net::IpOptname\">IpOptname</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"linux_object/signal/enum.Signal.html\" title=\"enum linux_object::signal::Signal\">Signal</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u8.html\">u8</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"linux_object/fs/vfs/struct.Timespec.html\" title=\"struct linux_object::fs::vfs::Timespec\">Timespec</a>&gt; for <a class=\"struct\" href=\"linux_object/time/struct.TimeSpec.html\" title=\"struct linux_object::time::TimeSpec\">TimeSpec</a>","synthetic":false,"types":["linux_object::time::TimeSpec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"linux_object/time/struct.TimeSpec.html\" title=\"struct linux_object::time::TimeSpec\">TimeSpec</a>&gt; for <a class=\"struct\" href=\"linux_object/fs/vfs/struct.Timespec.html\" title=\"struct linux_object::fs::vfs::Timespec\">Timespec</a>","synthetic":false,"types":["rcore_fs::vfs::Timespec"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"linux_object/time/struct.TimeSpec.html\" title=\"struct linux_object::time::TimeSpec\">TimeSpec</a>&gt; for <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/time/struct.Duration.html\" title=\"struct core::time::Duration\">Duration</a>","synthetic":false,"types":["core::time::Duration"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"linux_object/time/struct.TimeSpec.html\" title=\"struct linux_object::time::TimeSpec\">TimeSpec</a>&gt; for <a class=\"struct\" href=\"linux_object/time/struct.TimeVal.html\" title=\"struct linux_object::time::TimeVal\">TimeVal</a>","synthetic":false,"types":["linux_object::time::TimeVal"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"enum\" href=\"linux_object/time/enum.ClockId.html\" title=\"enum linux_object::time::ClockId\">ClockId</a>","synthetic":false,"types":["linux_object::time::ClockId"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"enum\" href=\"linux_object/time/enum.ClockFlags.html\" title=\"enum linux_object::time::ClockFlags\">ClockFlags</a>","synthetic":false,"types":["linux_object::time::ClockFlags"]}];
implementors["zcore_drivers"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zcore_drivers/prelude/enum.ColorFormat.html\" title=\"enum zcore_drivers::prelude::ColorFormat\">ColorFormat</a>&gt; for PixelFormatEnum","synthetic":false,"types":["sdl2::pixels::PixelFormatEnum"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"zcore_drivers/enum.DeviceError.html\" title=\"enum zcore_drivers::DeviceError\">DeviceError</a>","synthetic":false,"types":["zcore_drivers::DeviceError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zcore_drivers/prelude/enum.InputEventType.html\" title=\"enum zcore_drivers::prelude::InputEventType\">InputEventType</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u16.html\">u16</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;PropError&gt; for <a class=\"enum\" href=\"zcore_drivers/enum.DeviceError.html\" title=\"enum zcore_drivers::DeviceError\">DeviceError</a>","synthetic":false,"types":["zcore_drivers::DeviceError"]}];
implementors["zircon_object"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zircon_object/dev/pci/enum.PcieIrqMode.html\" title=\"enum zircon_object::dev::pci::PcieIrqMode\">PcieIrqMode</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zircon_object/dev/enum.ResourceKind.html\" title=\"enum zircon_object::dev::ResourceKind\">ResourceKind</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"kernel_hal/common/user/enum.Error.html\" title=\"enum kernel_hal::common::user::Error\">Error</a>&gt; for <a class=\"enum\" href=\"zircon_object/enum.ZxError.html\" title=\"enum zircon_object::ZxError\">ZxError</a>","synthetic":false,"types":["zircon_object::error::ZxError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"zircon_object/signal/struct.PortPacketRepr.html\" title=\"struct zircon_object::signal::PortPacketRepr\">PortPacketRepr</a>&gt; for <a class=\"struct\" href=\"zircon_object/signal/struct.PortPacket.html\" title=\"struct zircon_object::signal::PortPacket\">PortPacket</a>","synthetic":false,"types":["zircon_object::signal::port::port_packet::PortPacket"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;<a class=\"struct\" href=\"zircon_object/signal/struct.PortPacket.html\" title=\"struct zircon_object::signal::PortPacket\">PortPacket</a>&gt; for <a class=\"struct\" href=\"zircon_object/signal/struct.PortPacketRepr.html\" title=\"struct zircon_object::signal::PortPacketRepr\">PortPacketRepr</a>","synthetic":false,"types":["zircon_object::signal::port::port_packet::PortPacketRepr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zircon_object/task/enum.ThreadStateKind.html\" title=\"enum zircon_object::task::ThreadStateKind\">ThreadStateKind</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>","synthetic":false,"types":[]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"zircon_object/vm/enum.SeekOrigin.html\" title=\"enum zircon_object::vm::SeekOrigin\">SeekOrigin</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()