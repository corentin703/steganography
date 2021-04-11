use std::fs;
use std::io::{Error, ErrorKind, Result, Read, Write};

use crate::contained_file::ContainedFile;

// const SIZE_OFFSET_START_BMP: usize = 9;
// const SIZE_START: usize = 137;
const SIZE_START: usize = 73;

pub struct ContainerFile {
    v_file_buffer: Vec<u8>,
}

impl ContainerFile {
    pub fn from_file(mut file: fs::File) -> Result<ContainerFile> {

        if let Ok(metadata) = file.metadata() {

            let mut container_file = Self {
                v_file_buffer: vec![0; metadata.len() as usize],
            };

            file.read(&mut container_file.v_file_buffer)?;

            return Ok(container_file);
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Impossible de lire les métadonnées",
            ));
        }
    }

    pub fn get_content_size(&self) -> usize {
        self.v_file_buffer.len()
    }

    // fn get_start_offset(&self) -> usize {
    //     0
    // }

    pub fn save(&mut self, save_path: &str) -> Result<fs::File> {
        let mut file = fs::File::create(save_path);

        if let Ok(ref mut file) = file {
            file.write_all(&self.v_file_buffer)?;
        }

        file
    }

    pub fn encode(&mut self, source_file: &ContainedFile) -> Result<()> {
        // const SIZE_START: usize = 137;

        if source_file.get_max_size() > self.get_content_size() {
            Err(Error::new(ErrorKind::InvalidInput, "Taille du fichier source trop élevé"))
        } else {

            let source_buffer = source_file.get_buffer();
            let mut index = SIZE_START;
            let mut source_index = 0;

            while source_index < source_buffer.len() {
                index += 8;

                for i in 0..8 {
                    let index = index - i;
                    // index += 1;
                    self.v_file_buffer[index] = if (self.v_file_buffer[index] & 0b1) == 0b1 {
                        self.v_file_buffer[index] - 0b1
                    } else {
                        self.v_file_buffer[index]
                    };

                    self.v_file_buffer[index] += (source_buffer[source_index] >> i) & 0b1;
                }

                source_index += 1;
            }

            Ok(())
        }
    }

    pub fn decode(&self, dest_file: &mut ContainedFile) -> Result<()> {
        // const SIZE_START: usize = 137;

        if dest_file.get_max_size() < (self.get_content_size() / 8) {
            Err(Error::new(ErrorKind::InvalidInput, "Destination de taille trop faible"))
        } else {
            let mut buffer = 0;

            for index in SIZE_START..self.v_file_buffer.len() {
                buffer = match (index - SIZE_START) % 8 {
                    0 => {
                        buffer = buffer + (self.v_file_buffer[index] & 0b1);
                        dest_file.push(buffer)?;

                        0b0
                    },
                    i => {
                        buffer + ((self.v_file_buffer[index] & 0b1) << (8 - i))
                    },
                };
            }

            Ok(())
        }
    }
}