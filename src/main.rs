mod filemanip;
use clap::{App, Arg};

fn main() {
    let matches = App::new("file-bitflipper")
        .version("1.0")
        .author("Chloe <longwelldotpy@gmail.com>")
        .about("Bitflips images for fun")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .required(true)
                .help("Sets an input file to use")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("amount")
                .short("a")
                .long("amount")
                .required(true)
                .help("Sets the amount of bits to flip")
                .takes_value(true),
        )
        .get_matches();

    let path = matches.value_of("file").unwrap();
    let amount = matches.value_of("amount").unwrap().parse::<u32>().unwrap();
    let mut bytes = filemanip::load_file(path).expect("Could not find/load file");

    filemanip::flip_bits_in_binary_data(&mut bytes, amount);

    filemanip::output_modified_file(path, &bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flip_bit_works() {
        let chunk0: u8 = 0b1111_1111;
        let new_chunk0 = filemanip::util::flip_bit(chunk0, 2);

        assert_eq!(new_chunk0, 0b1111_1011);

        let chunk1: u8 = 0b0000_0000;
        let new_chunk1 = filemanip::util::flip_bit(chunk1, 5);

        assert_eq!(new_chunk1, 0b0010_0000);
    }
}
