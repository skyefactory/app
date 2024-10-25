use std::sync::{Arc,Mutex};
use crate::{input::Input,output::{Output, OutputType}};
#[derive(Default)]
pub struct Application {
    pub pid: u32,
    pub input: Arc<Mutex<Input>>,
    pub output: Arc<Mutex<Output>>,
    pub output_selection: OutputType,
}
impl Application {
    pub fn choose_output_type(&mut self, ctx: &eframe::egui::Context, ui: &mut eframe::egui::Ui) {
        ui.label(eframe::egui::RichText::new("Converting to").color(eframe::egui::Color32::RED));
        eframe::egui::ComboBox::from_label("Output Type")
            .selected_text(format!("{:?}", self.output_selection))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.output_selection,
                    OutputType::PNG,
                    OutputType::PNG.to_string(),
                );
                ui.selectable_value(
                    &mut self.output_selection,
                    OutputType::JPG,
                    OutputType::JPG.to_string(),
                );
                ui.selectable_value(
                    &mut self.output_selection,
                    OutputType::WEBM,
                    OutputType::WEBM.to_string(),
                );
            });
    }
}