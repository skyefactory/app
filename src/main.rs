//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

#[cfg(target_family = "windows")]
const FFMPEG : &str = "./ffmpeg-windows/bin/ffmpeg.exe";

#[cfg(target_family = "unix")]
const FFMPEG : &str = "./ffmpeg-linux/ffmpeg";

type Rezult = anyhow::Result<()>;
use eframe::egui::*;
use anyhow::anyhow;
use std::sync::{Arc,Mutex};
use futures::executor;
#[derive(Default)]
struct Application{
    pid: u32,
    picked_path: Arc<Mutex<Option<String>>>
}


fn initialize_libs() -> Rezult {
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Initialized");
    Ok(())
}

fn start_ui() -> Rezult{
    log::info!("Starting UI");
    let app = Application{
        pid: std::process::id(),
        picked_path: Arc::new(Mutex::new(None)),
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {

            Ok(Box::<Application>::new(app))
        }),
    ).expect("Failed to launch native window");
    Ok(())
}

fn check_for_ffmpeg() -> Rezult{
    use std::process::Command;
    let output = Command::new(FFMPEG).args(["-h"]).output().expect(format!("Failed to run {}", FFMPEG).as_str());
    if !output.status.success() {
        return Err(anyhow!("FFMPEG check failed"))
    }
    Ok(())
}
fn main() -> Rezult {
    initialize_libs()?;
    check_for_ffmpeg()?;

    
    start_ui()?;
    Ok(())
}

async fn get_file_path_dialog() -> String{
    use rfd::AsyncFileDialog;

    AsyncFileDialog::new()
    .pick_file()
    .await
    .unwrap()
    .path()
    .display()
    .to_string()
}

impl eframe::App for Application{
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            let path = self.picked_path.clone();
            ui.menu_button("File", |ui|{
                if ui.button("Open").clicked(){
                    log::info!("Open File");
                    
                    std::thread::spawn(move || {
                        let x = executor::block_on(get_file_path_dialog());
                        let mut path = path.lock().unwrap();
                        *path = Some(x);
                    });
                }
                
            })
         });
         eframe::egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(path)=self.picked_path.lock().unwrap().clone(){
                ui.label(path);
            }
            else{
                ui.label("No file selected...");
            }
         });
    }
}