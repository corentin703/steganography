use std::fs;
use std::io::{Error, ErrorKind, Result, Read, Write};
use std::path::Path;

use crate::container_file::ContainerFile;

/// Représente un fichier contenu dans une image bmp (un conteneur)
pub struct ContainedFile {

    /// Contient l'intégralité du fichier contenu
    file_buffer: Vec<u8>,

    /// Taille maximum du fichier (par rapport à la taille du conteneur)
    max_size: usize,
}

impl ContainedFile {

    /// Retourne un fichier contenu à d'une taille fournie
    ///
    /// # Arguments
    ///
    /// * `size` - Taille maximum du fichier (en octet)
    ///
    fn new(size: usize) -> Self {
        ContainedFile {
            file_buffer: Vec::new(),
            max_size: size,
        }
    }

    /// Crée et retourne une structure de type fichier contenu à partir de la taille d'un fichier contenant
    ///
    /// # Arguments
    ///
    /// * `file` - Fichier contenant à partir duquel récupèrer la taille
    ///
    pub fn from_container_file(file: &ContainerFile) -> Self {
        Self::new(file.get_content_size() / 8)
    }

    /// Crée et retourne une structure de type fichier contenu à partir d'un fichier
    ///
    /// # Arguments
    ///
    /// * `file` - Fichier à partir duquel instancier la structure de fichier contenu
    ///
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

    /// Retourne la taille maximum du fichier contenu
    pub fn get_max_size(&self) -> usize {
        self.max_size
    }

    /// Retourne le contenu du fichier
    pub fn get_buffer(&self) -> Vec<u8> {
        self.file_buffer.clone()
    }

    /// Ajoute un octet au fichier contenu
    pub fn push(&mut self, data: u8) -> Result<()> {

        if self.file_buffer.len() <= self.max_size {
            self.file_buffer.push(data);

            Ok(())
        } else {
            Err(Error::new(ErrorKind::InvalidInput, "Trop de données à insérer"))
        }
    }

    /// Sauvegarde le fichier vers la destination choisie
    ///
    /// # Arguments
    ///
    /// * `save_path` - Chemin auquel sauvegarder le fichier
    ///
    pub fn save(&self, save_path: &dyn AsRef<Path>) -> Result<fs::File> {
        let mut file = fs::File::create(save_path);

        if let Ok(ref mut file) = file {
            file.write_all(&self.file_buffer)?;
        }

        file
    }
}
