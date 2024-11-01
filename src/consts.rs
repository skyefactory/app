pub type Result = anyhow::Result<()>;
#[cfg(target_family = "windows")]
pub const FFMPEG: &str = "./ffmpeg-windows/bin/ffmpeg.exe";

#[cfg(target_family = "unix")]
pub const FFMPEG: &str = "./ffmpeg-linux/ffmpeg";
