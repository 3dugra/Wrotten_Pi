# Wrotten_Pi
Baremetal failure boilerplate for Raspberry Pi 4 Computer. Written mainly in Rust.

# Remember this project is for learning, and should not be taken into production code.


# To run/compile for first time
    -> When creating i used: 
    cargo new Wrotten_Pi --bin --edition 2021
    did not use 2024 because of the error "failed to parse manifest a (route)" Cause by: (feature "edition2024" is required)

    -> Then we add the target config:
    rustup target add armv-unknown-linux-gnueabihf

    -> Run the cargo for that target
    cargo build --target armv7-unknown-linux-gnueabihf
    by passing a --target we cross compile for a bare metal target system.


-> Remember to cd to the cargo
# From Version onwards of 22/02/2025
Step 1. Prepare the Toolchain & Target

Make sure you have installed a cross-compilation toolchain.
Add the ARM target:
    rustup target add arm-unknown-linux-gnueabihf
    (Note that we actually use the target armv7-unknown-none-eabihf in our configuration for baremetal work.)

Step 2. Build the Kernel

Open a Command Prompt and change to the Cargo root directory.

Run:
    set PATH=C:\msys64\mingw64\bin;%PATH% && cargo +nightly build

    This command builds your kernel into an ELF file.

Then choose:
    Manually convert the ELF file to a raw binary (which the Pi boot ROM expects):
    arm-none-eabi-objcopy -O binary your-kernel.elf kernel7.img

    (In our setup, the build script does this automatically and creates kernel7.img.)

Step 3. Assemble and Link the Bootloader

Assemble the bootloader source (boot.s):
    arm-none-eabi-as -o boot.o boot.s

Link the assembled object file (using 0x0 as the start address for the bootloader):
    arm-none-eabi-ld -Ttext=0x0 -o boot.elf boot.o
    (A warning about the entry symbol can be safely ignored, as long as your bootloader jumps to your kernel's address.)

Convert the linked bootloader to a binary
    arm-none-eabi-objcopy -O binary boot.elf bootloader.bin

Step 4. Concatenate the Bootloader and Kernel Images

Create the final bootable image by concatenating the bootloader and your kernel binary:
    copy /b bootloader.bin+kernel7.img final.img

This final image is what you'll write to your SD card.

Step 5. Run on the Raspberry Pi

Prepare an SD card with a FAT32 boot partition and copy the necessary Raspberry Pi firmware files (bootcode.bin, start4.elf, fixup4.dat, etc.).
Copy final.img (or rename it to kernel7.img if required by your firmware configuration) into the boot partition.
Insert the SD card into your Raspberry Pi 4 Model B and power it up.


Why Unsafe?
Due to direct memory writes (for example, writing to UART registers), Rust cannot prove safety. Hence, unsafe blocks are used to perform raw pointer accesses.