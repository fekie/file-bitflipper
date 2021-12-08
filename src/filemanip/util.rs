use rand;
use std::path::Path;

const MASKS: &'static [u8] = &[0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];

pub fn flip_bit(chunk: u8, index: usize) -> u8 {
    let mask = MASKS[index];
    chunk ^ u8::from(mask)
}

pub fn flip_random_bit(chunk: u8) -> u8 {
    let index = rand::random::<usize>() % 8;
    flip_bit(chunk, index)
}

// TODO: make this work with windows
pub fn get_file_name_without_extension(_path: &str) -> String {
    let name_with_ext = Path::new(_path).file_name().unwrap().to_str().unwrap();
    let name_without_ext = name_with_ext.split('.').next().unwrap();
    name_without_ext.to_string()
}

pub fn get_file_extension(path: &str) -> Option<String> {
    let ext_path_opt = Path::new(path).extension();

    let ext_string = match ext_path_opt {
        Some(ext) => Some(ext.to_string_lossy().into_owned()),
        None => None,
    };

    ext_string
}

/* pub fn flip_bit<T: std::ops::BitXor<Output = T> + From<u8>>(chunk: T, index: usize) -> T {
    let mask = MASKS[index];
    chunk ^ T::from(mask)
}
 */
