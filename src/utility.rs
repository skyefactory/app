use crate::{Application, Input, Output, OutputType, Result, FFMPEG};
use std::{sync::{Arc, Mutex}, io::Write};

pub fn initialize_libs() -> Result {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Initialized");
    Ok(())
}
pub fn start_ui() -> Result {
    log::info!("Starting UI");
    let app = Application {
        _pid: std::process::id(),
        input: Arc::new(Mutex::new(Input::default())),
        output: Arc::new(Mutex::new(Output::default())),
        output_selection: OutputType::default(),
        converting: false,
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
    use anyhow::anyhow;
    use std::process::Command;

    //Test if FFMPEG is present with our binary.
    let output = Command::new(FFMPEG).args(["-h"]).output();

    //Check the output from Command::new to determine if FFMPEG is present
    if let Ok(output) = output {
        //In this block, the process must have been started successfully using const FFMPEG
        log::info!("FFMPEG found: {}", FFMPEG);
        if !output.status.success() {
            //In this block, the process must have failed to run `ffmpeg -h`
            return Err(anyhow!("FFMPEG failed to run -h command"));
        } else {
            return Ok(());
        }
    } else {
        return Err(anyhow!("Could not find FFMPEG"));
    }
}
pub fn open_file(input: Arc<Mutex<Input>>) {
    use {futures::executor::block_on, log::info, std::thread::spawn};
    info!("Open File");
    //async file dialog setup
    //input is a (mutable, thread safe) reference to application.input
    //block on in a thread so main update can continue going
    spawn(move || block_on(input.lock().unwrap().get_file_from_user()));
}
pub fn save_file(output: Arc<Mutex<Output>>, ot: OutputType) {
    use {futures::executor::block_on, log::info, std::thread::spawn};
    info!("Saving File");
    //async file dialog setup
    //output is a (mutable, thread safe) reference to application.output
    //block on in a thread so main update can continue going
    spawn(move || block_on(output.lock().unwrap().save_file_dialog(ot)));
}
pub fn convert(input: Arc<Mutex<Input>>, output: Arc<Mutex<Output>>) -> Result {
    log::info!("Converting...");

    let mut input = input.lock().unwrap();
    let mut output = output.lock().unwrap();

    log::info!("Input: {:#?}", input);
    log::info!("Output: {:#?}", output);

    let command = &mut std::process::Command::new(FFMPEG);
    let command = command.args([
        format!("-i"),
        format!("{}", input.path().display()),
        format!("{}", output.path().display()),
    ]);
    *output = Output::default();

    let output = command.output();

    match output {
        Ok(out) => {
            std::io::stderr().write_all(&out.stderr).unwrap();
            log::info!("Successfully converted file.")
        }
        Err(out_err) => eprintln!("{}", out_err),
    }

    *input = Input::default();

    Ok(())
}