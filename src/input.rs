use crate::consts;
use std::path::{Path, PathBuf};
#[derive(Debug)]
pub struct Input {
    handle: Option<rfd::FileHandle>,
    full_path: PathBuf,
    name: String,
    ext: String,
}

fn get_ext_from_name(file_name: &String) -> String {
    let mut extension = format!("");
    for ch in file_name.chars() {
        if ch == '.' {
            extension = format!("");
        } else {
            extension.push(ch);
        }
    }
    extension.to_uppercase()
}

impl Input {
    pub async fn get_file_from_user(&mut self) -> consts::Result {
        match rfd::AsyncFileDialog::new().pick_file().await {
            Some(handle) => {
                self.handle = Some(handle.clone());
                self.full_path = handle.path().to_path_buf();
                self.name = handle.file_name();
                self.ext = get_ext_from_name(&handle.file_name());
                Ok(())
            }
            None => Err(anyhow::anyhow!("Failed to open file")),
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

impl Default for Input {
    fn default() -> Self {
        Self {
            handle: None,
            full_path: Default::default(),
            name: Default::default(),
            ext: Default::default(),
        }
    }
}
impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        self.full_path == other.full_path && self.name == other.name && self.ext == other.ext
    }
}
