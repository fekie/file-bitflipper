pub mod util;
use rand::Rng;
use std::fs;

pub fn load_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(path)
}

pub fn flip_bits_in_binary_data(data: &mut Vec<u8>, amount: u32) {
    let mut range = rand::thread_rng();
    for _ in 0..amount {
        let index = range.gen_range(0..data.len());
        let byte = data[index];
        let modified_byte = util::flip_random_bit(byte);
        data[index] = modified_byte;
    }
}

pub fn output_modified_file(old_path: &str, bytes: &[u8]) {
    let file_name = util::get_file_name_without_extension(old_path);
    let file_extension_opt = util::get_file_extension(old_path);
    let output_path = match file_extension_opt {
        Some(extension) => format!("{}_modified.{}", file_name, extension),
        None => format!("{}_modified", file_name),
    };
    //let output_path = format!("{}-bitflipped.{}", file_name, file_extension);
    fs::write(output_path, bytes).expect("Unable to write file");
}
