	.section .text.entry
	.globl _start
_start:
	#关中断
	csrw sie, zero

	#关闭mmu
        #csrw satp, zero

	#la sp, bootstacktop
	#call rust_main

	#可清零低12位地址
	lui t0, %hi(boot_page_table_sv39)
	li t1, 0xffffffffc0000000 - 0x40000000 #立即数加载
	#计算出页表的物理地址
	sub t0, t0, t1

	#右移12位，变为satp的PPN
	srli t0, t0, 12

	#C906物理地址共40位: PA = |--10--|--9--|--9--|--12--|
	#    虚拟地址共39位: VA =  |--9--|--9--|--9--|--12--|
	#SATP的PPN是28位，而非Qemu的44位

	#satp的MODE设为Sv39
	li t1, 8 << 60

	#写satp
	or t0, t0, t1
	csrw satp, t0

	#刷新TLB
	sfence.vma


	#此时在虚拟内存空间，设置sp为虚拟地址
	lui sp, %hi(bootstacktop)
	lui t0, %hi(rust_main)
	addi t0, t0, %lo(rust_main)
	jr t0

	.section .bss.stack
	.align 12
	.global bootstack
bootstack:
	.space 4096 * 32
	.global bootstacktop
bootstacktop:


	.section .data
	.align 12 #12位对齐
boot_page_table_sv39:
	#1G的一个大页: 0xffffffff_80000000 --> 0x00000000
	#1G的一个大页: 0xffffffff_C0000000 --> 0x40000000

	.zero 8

	# 0x40000000 --> 0x40000000
	#.quad (1 << 62) | (1 << 61) | (1 << 60) | (0x40000 << 10) | 0xef
	.quad (0x40000 << 10) | 0xef

	.zero 8 * 508

	#倒数第二项，PPN=0x40000(当转换为物理地址时还需左移12位), 标志位DAG_XWRV置1
	#c906扩展63:59位的页表项属性
	#.quad (1 << 62) | (1 << 61) | (1 << 60) | (0x00000 << 10) | 0xef
	#.quad (1 << 62) | (1 << 61) | (1 << 60) | (0x40000 << 10) | 0xef
	.quad (0x00000 << 10) | 0xef
	.quad (0x40000 << 10) | 0xef

