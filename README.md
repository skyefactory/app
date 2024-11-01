Simple UI over FFMPEG built using Egui and Rust.

Works on Linux & Windows, requires full shared build of ffmpeg present.
**ffmpeg must be in the same directory as the application executable, with the folder name "ffmpeg"**

cargo build --target x86_64-pc-windows-gnu
cargo build --target x86_64-unknown-linux-gnu