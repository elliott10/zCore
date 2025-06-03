# Makefile for top level of zCore

ARCH ?= x86_64

STRIP := $(ARCH)-linux-musl-strip
export PATH=$(shell printenv PATH):$(CURDIR)/ignored/target/$(ARCH)/$(ARCH)-linux-musl-cross/bin/

.PHONY: help zircon-init update rootfs libc-test other-test image check doc clean

# print top level help
help:
	cargo xtask

# download zircon binaries
zircon-init:
	cargo zircon-init

# update toolchain and dependencies
update:
	cargo update-all

# put rootfs for linux mode
rootfs:
ifeq ($(ARCH), riscv64)
	@mkdir -p rootfs/$(ARCH)/bin
	@mkdir -p rootfs/$(ARCH)/lib
	@ln -sf busybox rootfs/$(ARCH)/bin/ls
	@ln -sf busybox rootfs/$(ARCH)/bin/sh
	@[ -e rootfs/$(ARCH)/bin/busybox ] || \
		wget https://github.com/rcore-os/busybox-prebuilts/raw/master/busybox-1.30.1-riscv64/busybox -O rootfs/$(ARCH)/bin/busybox
	@echo Copy from $(musl_libc_dir)/lib/ld-musl-riscv64.so.1; cp $(musl_libc_dir)/lib/libc.so rootfs/riscv64/lib/ld-musl-riscv64.so.1

musl_libc_dir := $(shell riscv64-linux-musl-gcc -print-sysroot)

else ifeq ($(ARCH), x86_64)
	@mkdir -p rootfs/$(ARCH)
	@[ -e rootfs/alpine-minirootfs-3.12.0-x86_64.tar.gz ] || \
		wget http://dl-cdn.alpinelinux.org/alpine/v3.12/releases/x86_64/alpine-minirootfs-3.12.0-x86_64.tar.gz -O rootfs/alpine-minirootfs-3.12.0-x86_64.tar.gz
	@[ -e rootfs/$(ARCH)/bin/busybox ] || tar xf rootfs/alpine-minirootfs-3.12.0-x86_64.tar.gz -C rootfs/$(ARCH)
	@cp -rf rootfs/$(ARCH) rootfs/libos
# libc-libos.so (convert syscall to function call) is from https://github.com/rcore-os/musl/tree/rcore
	@cp prebuilt/linux/libc-libos.so rootfs/libos/lib/ld-musl-x86_64.so.1

else ifeq ($(ARCH), aarch64)
	@[ -e rootfs/testsuits-aarch64-linux-musl.tgz ] || \
		wget https://github.com/rcore-os/testsuits-for-oskernel/releases/download/final-20240222/testsuits-aarch64-linux-musl.tgz -O rootfs/testsuits-aarch64-linux-musl.tgz
	@[ -e rootfs/$(ARCH)/busybox ] || tar xf rootfs/testsuits-aarch64-linux-musl.tgz  -C rootfs/
	@[ -e rootfs/$(ARCH)/busybox ] || mv rootfs/testsuits-aarch64-linux-musl rootfs/$(ARCH)
	@ln -sf busybox rootfs/$(ARCH)/bin/ls
	@cp rootfs/$(ARCH)/busybox rootfs/$(ARCH)/bin/
else
	cargo rootfs --arch $(ARCH)
endif

rcore_fs_fuse_revision := 98cfeec

rcore-fs-fuse:
ifneq ($(shell rcore-fs-fuse dir image git-version), $(rcore_fs_fuse_revision))
	@echo Installing rcore-fs-fuse
	@cd ~ && cargo install rcore-fs-fuse --git https://github.com/elliott10/rcore-fs --rev $(rcore_fs_fuse_revision) --force
endif

# put libc tests into rootfs
libc-test:
	cargo libc-test --arch $(ARCH)
	find rootfs/$(ARCH)/libc-test -type f \
	       -name "*so" -o -name "*exe" -exec $(STRIP) {} \; 

# put other tests into rootfs
other-test:
	cargo other-test --arch $(ARCH)

# build image from rootfs
image: rcore-fs-fuse rootfs
ifneq ($(filter $(ARCH),riscv64 aarch64 x86_64),)
	@echo Creating zCore/$(ARCH).img
	@rcore-fs-fuse zCore/$(ARCH).img rootfs/$(ARCH) zip
	@qemu-img resize -f raw zCore/$(ARCH).img +200K
else
	cargo image --arch $(ARCH)
endif

# check code style
check:
	cargo check-style

# build and open project document
doc:
	cargo doc --open

# clean targets
clean:
	cargo clean
	rm -f  *.asm
	rm -rf rootfs
	rm -rf zCore/disk
	find zCore -maxdepth 1 -name "*.img" -delete
	find zCore -maxdepth 1 -name "*.bin" -delete

# delete targets, including those that are large and compile slowly
cleanup: clean
	rm -rf ignored/target

# delete everything, including origin files that are downloaded directly
clean-everything: clean
	rm -rf ignored

# rt-test:
# 	cd rootfs/x86_64 && git clone https://kernel.googlesource.com/pub/scm/linux/kernel/git/clrkwllms/rt-tests --depth 1
# 	cd rootfs/x86_64/rt-tests && make
# 	echo x86 gcc build rt-test,now need manual modificy.
