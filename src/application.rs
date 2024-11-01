use crate::{
    input::Input,
    output::{Output, OutputType},
};
use std::sync::{Arc, Mutex};
#[derive(Default)]
pub struct Application {
    pub _pid: u32,
    pub input: Arc<Mutex<Input>>,
    pub output: Arc<Mutex<Output>>,
    pub output_selection: OutputType,
    pub converting: bool,
}
impl Application {
    pub fn choose_output_type(&mut self, _ctx: &eframe::egui::Context, ui: &mut eframe::egui::Ui) {
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
                    OutputType::GIF,
                    OutputType::GIF.to_string(),
                );
                ui.selectable_value(
                    &mut self.output_selection,
                    OutputType::SVG,
                    OutputType::SVG.to_string(),
                );
                ui.selectable_value(
                    &mut self.output_selection,
                    OutputType::RAW,
                    OutputType::RAW.to_string(),
                );
            });
    }
}
