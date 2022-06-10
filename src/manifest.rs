use serde::{Deserialize, Serialize};

use crate::error::EcstasyError;

use log::debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcstasyManifestItem {
    pub url: String,
    pub file: String,
    pub tags: Vec<String>,
}

impl EcstasyManifestItem {
    pub fn new(url: String, file: String, tags: Vec<String>) -> Self {
        Self { url, file, tags }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcstasyManifest {
    pub files: Vec<EcstasyManifestItem>,
    pub downloader: String,
}

impl EcstasyManifest {
    pub fn new(downloader: String) -> Self {
        Self {
            files: Vec::new(),
            downloader,
        }
    }

    fn _get_path(&self) -> Result<String, EcstasyError> {
        let folder = format!("downloads/{}", &self.downloader);
        if !std::path::Path::new(&folder).exists() {
            debug!(
                "Manifest folder for {} doesn't exist, creating it.",
                &self.downloader
            );
            std::fs::create_dir_all(&folder)?;
        }
        Ok(format!("{}/manifest.json.gz", folder))
    }

    pub fn add(&mut self, _item: EcstasyManifestItem) -> Self {
        self.clone()
    }

    pub fn load(&self) -> Result<Self, EcstasyError> {
        Ok(self.to_owned())
    }

    pub fn save(&self) -> Result<(), EcstasyError> {
        Ok(())
    }
}
