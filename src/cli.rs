use std::fs;
use std::io::Result;
use std::path::PathBuf;

use structopt::StructOpt;
use steganography::{ContainedFile, ContainerFile};


#[derive(StructOpt, Debug)]
enum SteganographyMode {
    Decode,
    Encode {
        container_file_path: PathBuf
    },
}


#[derive(StructOpt, Debug)]
#[structopt(name = "steganography", about = "Un programme pour cacher des fichiers dans une image")]
struct SteganographyCliArgs {
    /// Mode (encodage ou décodage)
    #[structopt(subcommand)]
    mode: SteganographyMode,

    /// Fichier d'entrée
    #[structopt(short = "-i", long = "--input", parse(from_os_str))]
    input_file_path: PathBuf,

    /// Fichier de sortie
    #[structopt(short = "-o", long = "--output", parse(from_os_str))]
    output_file_path: PathBuf,
}

pub fn run() -> Result<()> {
    let args = SteganographyCliArgs::from_args();

    match args.mode {
        SteganographyMode::Encode { container_file_path } => {
            let container_file = fs::File::open(container_file_path)?;
            let mut container_file = ContainerFile::from_file(container_file)?;

            let source_file = fs::File::open(args.input_file_path)?;
            let source_file = ContainedFile::from_file(source_file)?;

            container_file.encode(&source_file)?;
            container_file.save(&args.output_file_path)?;
        }
        SteganographyMode::Decode => {
            let container_file = fs::File::open(args.input_file_path)?;
            let container_file = ContainerFile::from_file(container_file)?;

            let mut contained_file = ContainedFile::from_container_file(&container_file);

            container_file.decode(&mut contained_file)?;
            contained_file.save(&args.output_file_path)?;
        }
    }

    Ok(())
}
