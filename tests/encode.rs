#[cfg(test)]

use std::fs;

use steganography::ContainedFile;
use steganography::ContainerFile;


#[test]
fn test_file_enc() {
    let source_file = fs::OpenOptions::new().read(true).open("tests/res/test_enc_01.txt").unwrap();
    let mut source_file = ContainedFile::from_file(source_file).unwrap();

    let container_file = fs::OpenOptions::new().read(true).open("tests/res/test_enc_01.bmp").unwrap();
    let mut container_file = ContainerFile::from_file(container_file).unwrap();
    container_file.encode(&mut source_file).unwrap();

    fs::create_dir("target/tests");
    container_file.save(&"target/tests/test_enc_01.bmp").unwrap();

    let container_file = fs::OpenOptions::new().read(true).open("target/tests/test_enc_01.bmp").unwrap();
    let container_file = ContainerFile::from_file(container_file).unwrap();

    let mut contained_file = ContainedFile::from_container_file(&container_file);

    container_file.decode(&mut contained_file).unwrap();

    // container_file.save(&"target/tests/test_enc_01.txt").unwrap();

    unsafe {
        let str = String::from_utf8_unchecked(contained_file.get_buffer());
        assert!(str.contains("Super ça marche !!"));
    }
}
