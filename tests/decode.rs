#[cfg(test)]

use std::fs;

use steganography::ContainedFile;
use steganography::ContainerFile;


#[test]
fn test_file_dec() {
    let container_file = fs::OpenOptions::new().read(true).open("tests/res/test_dec_01.bmp").unwrap();
    let container_file = ContainerFile::from_file(container_file).unwrap();

    fs::create_dir("target/tests");
    let mut contained_file = ContainedFile::from_container_file(&container_file);

    container_file.decode(&mut contained_file).unwrap();

    contained_file.save(&"target/tests/test_dec_01.txt").unwrap();

    unsafe {
        let str = String::from_utf8_unchecked(contained_file.get_buffer());
        assert!(str.contains("Super ça marche !!"));
    }

}
