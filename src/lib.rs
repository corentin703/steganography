mod contained_file;
mod container_file;

use std::fs;
use std::io::Result;

pub use contained_file::ContainedFile;
pub use container_file::ContainerFile;

pub fn run(s_container_file_path: &str, s_contained_file_path: &str) -> Result<()> {
    let file = fs::OpenOptions::new().read(true).open(s_container_file_path)?;

    let picture = container_file::ContainerFile::from_file(file)?;
    let mut new_file = contained_file::ContainedFile::new(picture.get_content_size());

    picture.decode(&mut new_file)?;
    new_file.save(s_contained_file_path)?;

    Ok(())
}
