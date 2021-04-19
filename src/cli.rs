use std::fs;
use std::io::Result;
use std::path::PathBuf;

use structopt::StructOpt;
use steganography::{ContainedFile, ContainerFile};


#[derive(StructOpt, Debug)]
#[structopt(
    name = "steganography",
    about = "Un programme pour cacher des fichiers dans une image",
    author = "Corentin VÉROT",
)]
enum SteganographyCliArgs {
    /// Mode décodage
    Decode {
        /// Fichier d'entrée
        #[structopt(short = "-i", long = "--input", parse(from_os_str))]
        input_file_path: PathBuf,

        /// Fichier de sortie
        #[structopt(short = "-o", long = "--output", parse(from_os_str))]
        output_file_path: PathBuf,
    },

    /// Mode encodage
    Encode {
        /// Fichier bmp conteneur (dans lequel stocker les données à cacher)
        #[structopt(short = "-c", long = "--container", parse(from_os_str))]
        container_file_path: PathBuf,

        /// Fichier d'entrée
        #[structopt(short = "-i", long = "--input", parse(from_os_str))]
        input_file_path: PathBuf,

        /// Fichier de sortie
        #[structopt(short = "-o", long = "--output", parse(from_os_str))]
        output_file_path: PathBuf,
    },
}

pub fn run() -> Result<()> {
    let args = SteganographyCliArgs::from_args();

    match args {
        SteganographyCliArgs::Encode { container_file_path, input_file_path, output_file_path } => {
            let container_file = fs::File::open(container_file_path)?;
            let mut container_file = ContainerFile::from_file(container_file)?;

            let source_file = fs::File::open(input_file_path)?;
            let source_file = ContainedFile::from_file(source_file)?;

            container_file.encode(&source_file)?;
            container_file.save(&output_file_path)?;
        }
        SteganographyCliArgs::Decode { input_file_path, output_file_path } => {
            let container_file = fs::File::open(input_file_path)?;
            let container_file = ContainerFile::from_file(container_file)?;

            let mut contained_file = ContainedFile::from_container_file(&container_file);

            container_file.decode(&mut contained_file)?;
            contained_file.save(&output_file_path)?;
        }
    }

    Ok(())
}
