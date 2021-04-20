use std::fs;
use std::io::{Error, ErrorKind, Result, Read, Write};
use std::path::Path;

use crate::contained_file::ContainedFile;

/// Numéro de l'octet à partir duquel écraser / lire les LSB
const SIZE_START: usize = 137;

/// Représente un fichier contenant
pub struct ContainerFile {

    /// Contient l'intégralité du fichier contenant
    v_file_buffer: Vec<u8>,
}

impl ContainerFile {

    /// Crée et retourne une structure de type fichier contenant à partir d'un fichier
    ///
    /// # Arguments
    ///
    /// * `file` - Fichier à partir duquel instancier la structure de fichier contenant
    ///
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

    /// Retourne le nombre d'octet du fichier contenant
    pub fn get_content_size(&self) -> usize {
        self.v_file_buffer.len()
    }

    /// Sauvegarde le fichier vers la destination choisie
    ///
    /// # Arguments
    ///
    /// * `save_path` - Chemin auquel sauvegarder le fichier
    ///
    pub fn save(&mut self, save_path: &dyn AsRef<Path>) -> Result<fs::File> {
        let mut file = fs::File::create(save_path);

        if let Ok(ref mut file) = file {
            file.write_all(&self.v_file_buffer)?;
        }

        file
    }

    /// Encode / cache un fichier dans le conteneur
    ///
    /// # Arguments
    ///
    /// * `source_file` - Fichier à cacher (fichier contenu)
    ///
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

    /// Décode / récupère un fichier caché dans le conteneur
    ///
    /// # Arguments
    ///
    /// * `dest_file` - Fichier contenu à remplir avec les octets récupérés
    ///
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