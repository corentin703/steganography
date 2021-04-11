#[cfg(test)]

use std::fs;

use steganography::ContainedFile;
use steganography::ContainerFile;

#[test]
fn test_file_dec() {
    // let mut file = open_picture("/home/corentin/Bureau/mod.bmp").unwrap();
    let file = fs::OpenOptions::new().read(true).open("tests/res/mod.bmp").unwrap();
    let file = ContainerFile::from_file(file).unwrap();

    let mut new_file = ContainedFile::from_container_file(&file);

    file.decode(&mut new_file).unwrap();

    unsafe {
        let str = String::from_utf8_unchecked(new_file.get_buffer());
        assert!(str.contains("Salut les amis"));
    }

    // new_file.save("target/tests/mod.txt").unwrap();
}

// #[test]
// fn dec_video_monc() {
//     // let mut file = open_picture("/home/corentin/Bureau/mod.bmp").unwrap();
//     let file = fs::OpenOptions::new().read(true).open("/home/corentin/Bureau/img.bmp").unwrap();
//     let file = ContainerFile::from_file(file).unwrap();
//
//     let mut new_file = ContainedFile::new(file.get_content_size());
//
//     file.decode(&mut new_file).unwrap();
//     new_file.save("/home/corentin/Bureau/vid.txt").unwrap();
// }
