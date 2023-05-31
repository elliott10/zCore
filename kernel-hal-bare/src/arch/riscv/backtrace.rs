//
// By benpichu@gmail.com
//
use crate::putfmt;
use addr2line::gimli;
use addr2line::Context;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use core::option::Option;
use core::slice;
use lazy_static::*;
use object::{File, Object, ObjectSection};
use trapframe::TrapFrame;

pub fn print_backtrace(tf: &TrapFrame) {
    bare_print!("\n\u{1B}[31m");
    extern "C" {
        fn stext();
        fn etext();
    }
    let stext = stext as usize;
    let etext = etext as usize;
    let mut fp: usize;
    let mut ra: usize;
    /* by xly
    unsafe {
        llvm_asm!("mv $0,fp;auipc $1,0x0"
            : "=r" (fp),"=r" (ra)
            :
            :
            : "volatile"
        );
    }
    */

    fp = tf.general.s0; //寄存器x8是帧指针
    ra = tf.sepc;

    bare_println!(
        "fp: {:#x}, ra: {:#x}, .text section: {:#x?}-{:#x?}",
        fp,
        ra,
        stext,
        etext
    );
    bare_println!("stack: {:#x?}", tf.general.sp);

    // 处理sepc跑飞的情况
    if ra < stext {
        ra = stext;
    } else if ra > etext {
        ra = etext;
    }

    let mut layer = 0usize;
    while stext <= ra && ra <= etext && fp != 0x0 {
        bare_println!("{:?}: {:#x?}", layer, ra);
        let mut frames = ADDR2LINE_CONTEXT
            .as_ref()
            .and_then(|ctx| ctx.context.find_frames(ra as u64).ok());
        if let Some(frames) = frames.as_mut() {
            while let Ok(Some(frame)) = frames.next() {
                bare_print!("    ");
                if let Some(function) = frame.function {
                    let name = function.demangle();
                    bare_print!(
                        "{} ",
                        name.as_ref()
                            .map_or("<Error getting name>", |name| name.as_ref())
                    );
                }
                if let Some(location) = frame.location {
                    bare_print!("at {}", location.file.unwrap_or("<Error getting file>"));
                    if let Some(line) = location.line {
                        bare_print!(":{}", line);
                        if let Some(column) = location.column {
                            bare_print!(":{}", column);
                        }
                    }
                }
                #[allow(clippy::println_empty_string)]
                bare_println!("")
            }
        }
        bare_println!("    fp: {:#x?}", fp);
        if fp < stext {
            bare_println!("    fp: {:#x?} out of bounds", fp);
            break;
        }

        unsafe {
            ra = (fp as *mut usize).offset(-1).read_volatile();
            fp = (fp as *mut usize).offset(-2).read_volatile();
        }
        layer += 1;
    }
    bare_print!("\u{1B}[0m\n");
}

struct Addr2LineContext {
    context: Context<gimli::EndianSlice<'static, gimli::RunTimeEndian>>,
}

unsafe impl Sync for Addr2LineContext {}

fn load_debuginfo() -> Option<Addr2LineContext> {
    let debuginfo_range =
        //unsafe { slice::from_raw_parts_mut(_kernel_debuginfo_start as *mut u8, _kernel_debuginfo_end as usize - _kernel_debuginfo_start as usize) };
        unsafe { slice::from_raw_parts_mut(DEBUGINFO_ELF_ADDRESS as *mut u8, DEBUGINFO_ELF_SIZE) };
    let debuginfo_elf = File::parse(debuginfo_range).ok()?;
    let endian = if debuginfo_elf.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let load_section = |id: gimli::SectionId| -> Result<_, gimli::Error> {
        match debuginfo_elf.section_by_name(id.name()).as_ref() {
            Some(section) => Ok(section
                .uncompressed_data()
                .unwrap_or(Cow::Borrowed(&[][..]))),
            None => Ok(Cow::Borrowed(&[][..])),
        }
    };
    let load_sep_section = |_| Ok(Cow::Borrowed(&[][..]));
    let dwarf_cow = Box::leak(Box::new(
        gimli::Dwarf::load(load_section, load_sep_section).ok()?,
    ));
    let borrow_section: &dyn for<'a> Fn(
        &'a Cow<[u8]>,
    ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(&*section, endian);
    let dwarf = dwarf_cow.borrow(&borrow_section);
    addr2line::Context::from_dwarf(dwarf)
        .map(|context| Addr2LineContext { context })
        .ok()
}

/// Hard link kernel debuginfo ELF
#[cfg(feature = "link_kdbg")]
global_asm!(concat!(
    r#"
	.section .rodata.kdbg
	.global _kernel_debuginfo_start
	.global _kernel_debuginfo_end
_kernel_debuginfo_start:
    .incbin ""#,
    env!("KDBG"),
    r#""
_kernel_debuginfo_end:
"#
));

#[cfg(feature = "link_kdbg")]
extern "C" {
    fn _kernel_debuginfo_start();
    fn _kernel_debuginfo_end();
}

pub const DEBUGINFO_ELF_ADDRESS: usize = 0x45000000; //这里不使用。将来考虑把内核符号信息文件放在指定地址
pub const DEBUGINFO_ELF_SIZE: usize = 0x200000;

lazy_static! {
    //static ref ADDR2LINE_CONTEXT: Option<Addr2LineContext> = load_debuginfo();
    static ref ADDR2LINE_CONTEXT: Option<Addr2LineContext> = None;
}

pub fn init() {
    extern "C" {
        fn skernel();
        fn ekernel();
    }
    bare_println!("kernel: {:#x?}-{:#x?}", skernel as usize, ekernel as usize);
    bare_println!("addr2line ok? {:?}", ADDR2LINE_CONTEXT.is_some());
}
