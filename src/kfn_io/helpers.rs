use std::fmt::Write;

pub fn dump_hex(array: Vec<u8>) -> String {
    let mut output = String::new();
    for i in 0..array.len() {
        if i > 0 {
            output.push(' ');
        }
        let mut tmp = String::new();
        write!(&mut tmp, "{:X} ", array[i] & 0xFF);
        output.push_str(&tmp);
    }
    output
}