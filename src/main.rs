use ffmpeg_next as ffmpeg;

type Rezult = anyhow::Result<()>;

fn initialize_libs() -> Rezult {
    ffmpeg::init()?;
    pretty_env_logger::init();
    log::info!("Initialized");
    Ok(())
}

fn main() -> Rezult {
    initialize_libs()?;
    Ok(())
}
