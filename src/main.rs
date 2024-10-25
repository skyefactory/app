//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod consts;
mod input;
mod output;
mod utility;
mod application;

use {application::*, consts::*, input::*, output::*, utility::*};
use std::sync::{Arc,Mutex};

    fn main() -> consts::Result {
    initialize_libs()?;
    check_for_ffmpeg()?;
    start_ui()?;

    Ok(())
}

fn open_file(input: Arc<Mutex<Input>>) {
    use {log::info,futures::executor::block_on, std::thread::spawn};
    info!("Open File");
    //async file dialog setup
    //input is a (mutable, thread safe) reference to application.input
    //block on in a thread so main update can continue going
    spawn(move || block_on(input.lock().unwrap().get_file_from_user()));
}

fn save_file(output: Arc<Mutex<Output>>, ot: OutputType) {
    use {log::info,futures::executor::block_on, std::thread::spawn};
    info!("Saving File");
    //async file dialog setup
    //output is a (mutable, thread safe) reference to application.output
    //block on in a thread so main update can continue going
    spawn(move || block_on(output.lock().unwrap().save_file_dialog(ot)));
}


//Update loop
impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        use eframe::egui::*;

        CentralPanel::default().show(ctx, |ui| {

            //Mutable, thread safe references to the applications input and output structures
            let inref = Arc::clone(&self.input);
            let outref = Arc::clone(&self.output);

            //Spawn a dialog box for user to choose input file. 
            if ui.button("Open File").clicked() {
                open_file(inref)
                //inref is taken by function, need a new one if using again.
            }

            //Check to see if the 'Open File' thread is still active with try_lock() on the Mutex
            if let Ok(input) = Arc::clone(&self.input).try_lock() {
                //File is not locked, check to see if there is a valid file handle in the Input structure
                if input.is_file_ready() {
                    ui.label(format!("{:?}", input));
                    ui.label("Select output type");
                    self.choose_output_type(ctx, ui);
                    if self.output_selection != OutputType::default() {
                        if ui.button("Convert!").clicked() {
                            save_file(outref, self.output_selection.clone())
                            //outref is taken by function, need a new one if using again.
                        }

                        if let Ok(output) = Arc::clone(&self.output).try_lock() {
                            if output.is_file_ready() {
                                ui.label(format!("{:?}", output));
                            }
                        }
                    }
                } else {
                    ui.label("No file has been selected");
                }
            }
        });
    }
}
