#[cfg(test)]

use std::fs;

use steganography::ContainedFile;
use steganography::ContainerFile;


#[test]
fn test_file_enc() {
    let source_file = fs::OpenOptions::new().read(true).open("tests/res/test.txt").unwrap();
    let mut source_file = ContainedFile::from_file(source_file).unwrap();

    let dest_file = fs::OpenOptions::new().read(true).open("tests/res/an.bmp").unwrap();
    let mut dest_file = ContainerFile::from_file(dest_file).unwrap();
    dest_file.encode(&mut source_file).unwrap();

    fs::create_dir("target/tests");
    dest_file.save("target/tests/test.bmp").unwrap();

    let dest_file = fs::OpenOptions::new().read(true).open("target/tests/test.bmp").unwrap();
    let dest_file = ContainerFile::from_file(dest_file).unwrap();
    // let mut new_file = ContainedFile::from_container_file(&dest_file);

    let mut new_file = ContainedFile::from_container_file(&dest_file);

    dest_file.decode(&mut new_file).unwrap();

    new_file.save("target/tests/test_dec.txt").unwrap();

    unsafe {
        let str = String::from_utf8_unchecked(new_file.get_buffer());
        assert!(str.contains("Super ça marche !!"));
    }
}
