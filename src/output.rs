use crate::consts;
use std::{fmt::{Display,Formatter,Result},path::PathBuf};
use rfd::FileHandle;

#[derive(PartialEq, Debug, Clone)]
pub enum OutputType {
    NotSelected,
    PNG,
    JPG,
    WEBM,
}
impl Default for OutputType {
    fn default() -> Self {
        Self::NotSelected
    }
}

impl Display for OutputType {
    fn fmt(&self, f: &mut Formatter) ->Result {
        write!(f, "{:?}", self)
    }
}
#[derive(Debug)]
pub struct Output {
    handle: Option<FileHandle>,
    full_path: PathBuf,
    name: String,
    ext: String,
}

impl Default for Output {
    fn default() -> Self {
        Self {
            handle: None,
            full_path: Default::default(),
            name: Default::default(),
            ext: Default::default(),
        }
    }
}

impl Output {
    pub fn set_ext(&mut self, new_ext: String) {
        self.ext = new_ext;
    }
    pub fn new(file_type: String) -> Self {
        let mut retrn = Self::default();
        retrn.ext = file_type;
        retrn
    }
    pub async fn save_file_dialog(&mut self, ext: OutputType) -> consts::Result {
        use rfd::AsyncFileDialog;
        use anyhow::anyhow;
        self.set_ext(ext.to_string());
        match AsyncFileDialog::new()
            .add_filter("Output", &[format!("{}", self.ext)])
            .set_directory("/")
            .save_file()
            .await
        {
            Some(handle) => {
                self.handle = Some(handle.clone());
                self.full_path = handle.path().to_path_buf();
                self.name = handle.file_name();
                Ok(())
            }
            None => Err(anyhow!("Failed to save file")),
        }
    }
    pub fn is_file_ready(&self) -> bool {
        self.handle.is_some()
    }
    pub fn path(&self) -> PathBuf {
        self.full_path.clone()
    }
    pub fn extension(&self) -> String {
        self.ext.clone()
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
