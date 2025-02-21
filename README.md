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