use::std::sync::{Mutex,Arc};

pub fn initialize_libs() -> Result {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Initialized");
    Ok(())
}
pub fn start_ui() -> Result {
    log::info!("Starting UI");
    let app = Application {
        pid: std::process::id(),
        input: Arc::new(Mutex::new(input::Input::default())),
        output: Arc::new(Mutex::new(output::Output::default())),
        output_selection: OutputType::default(),
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "App",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<Application>::new(app))
        }),
    )
    .expect("Failed to launch native window");
    Ok(())
}
pub fn check_for_ffmpeg() -> Result {
    use std::process::Command;
    use anyhow::anyhow;

    //Test if FFMPEG is present with our binary.
    let output = Command::new(FFMPEG)
        .args(["-h"])
        .output();

    //Check the output from Command::new to determine if FFMPEG is present
    if let Ok(output) = output{
        //In this block, the process must have been started successfully using const FFMPEG
        log::info!("FFMPEG found: {}", FFMPEG);
        if !output.status.success() {
            //In this block, the process must have failed to run `ffmpeg -h`
            return Err(anyhow!("FFMPEG failed to run -h command"));
        }else{
            return Ok(())
        }
    }
    else{
        return Err(anyhow!("Could not find FFMPEG"))
    }

    

    Ok(())
}
