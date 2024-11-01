//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unused)]
mod application;
mod consts;
mod input;
mod output;
mod utility;

use std::{
    io::Write,
    sync::{Arc, Mutex},
};
use {application::*, consts::*, input::*, output::*, utility::*};

fn main() -> Result {
    initialize_libs()?;
    check_for_ffmpeg()?;
    start_ui()?;

    Ok(())
}



//Update loop
impl eframe::App for Application {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
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
                            save_file(outref, self.output_selection.clone());
                            //outref is taken by function, need a new one if using again.
                        }
                    }
                } else {
                    ui.label("No file has been selected");
                }
            }

            if let Ok(out) = Arc::clone(&self.output).try_lock() {
                if out.is_file_ready() {
                    self.converting = true;
                }
            }

            if self.converting {
                self.converting = false;
                convert(Arc::clone(&self.input), Arc::clone(&self.output));
            }
        });
    }
}
