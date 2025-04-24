fn main() {
    let file = include_bytes!("./sa0a1d081b8a108bb8c9847c4cd83db662.sar");
    let sar = sar_parser_core::parser::payload::parse(Vec::from(file).into()).unwrap();

    println!("{:?}", sar);
    std::fs::write("./sar-core/examples/result.txt", format!("{:?}", sar)).unwrap();
}
