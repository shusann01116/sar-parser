use std::fs;

use crate::parser::parse;

pub const RAW_FILE: &[u8] = include_bytes!("../../fixture/sa0a1d081b8a108bb8c9847c4cd83db662.sar");

#[test]
fn test_something() {
    let bytes = Box::from(RAW_FILE);
    let payload = parse(bytes).unwrap();
    // save result to file
    fs::write("./result.txt", format!("{:?}", payload)).unwrap();
}
