use std::fs;
use std::io::{Error, ErrorKind, Result, Read, Write};

use crate::container_file::ContainerFile;

pub struct ContainedFile {
    file_buffer: Vec<u8>,
    max_size: usize,
}

impl ContainedFile {
    pub fn new(size: usize) -> Self {
        ContainedFile {
            file_buffer: Vec::new(),
            max_size: size,
        }
    }

    pub fn from_container_file(file: &ContainerFile) -> Self {
        Self::new(file.get_content_size() / 8)
    }

    pub fn from_file(mut file: fs::File) -> Result<Self> {

        if let Ok(metadata) = file.metadata() {

            let mut contained_file = Self {
                file_buffer: vec![0; metadata.len() as usize],
                max_size: metadata.len() as usize,
            };

            file.read(&mut contained_file.file_buffer)?;
            contained_file.max_size = contained_file.file_buffer.len();

            Ok(contained_file)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Impossible de lire les métadonnées",
            ));
        }
    }

    pub fn get_max_size(&self) -> usize {
        self.max_size
    }

    pub fn get_buffer(&self) -> Vec<u8> {
        self.file_buffer.clone()
    }

    pub fn push(&mut self, data: u8) -> Result<()> {

        if self.file_buffer.len() <= self.max_size {
            self.file_buffer.push(data);

            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Trop de données à insérer"))
        }
    }

    pub fn save(&self, save_path: &str) -> Result<fs::File> {
        let mut file = fs::File::create(save_path);

        if let Ok(ref mut file) = file {
            file.write_all(&self.file_buffer)?;
        }

        file
    }
}
