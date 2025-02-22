use std::process::Command;

fn main() {
    // The output ELF from Cargo should be in OUT_DIR; adjust the path as needed.
    // Here we assume the final ELF is named "Wrotten_Pi"
    let status = Command::new("arm-none-eabi-objcopy")
        .args(&["-O", "binary", "target/target_rasp/debug/Wrotten_Pi", "kernel7.img"])
        .status()
        .expect("failed to execute arm-none-eabi-objcopy");

    if !status.success() {
        panic!("objcopy failed");
    }
}