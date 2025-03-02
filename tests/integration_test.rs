use std::io::{Read, Seek};

use simple_arithmetic_coding::{decode_routine, encode_routine, ArithmeticStreamDecoder, ArithmeticStreamEncoder};

#[test]
fn test_iterators() {
    let file = std::fs::File::open("./war_and_peace.txt").unwrap();
    let mut bufreader = std::io::BufReader::new(&file);

    let arienc = ArithmeticStreamEncoder::new(&mut bufreader)
        .unwrap()
        .collect::<Vec<u8>>();

    bufreader.rewind().unwrap();
    let mut normal = Vec::new();
    bufreader.read_to_end(&mut normal).unwrap();

    assert_ne!(normal, arienc);

    let mut arienc = arienc.as_ref();
    let decoder = ArithmeticStreamDecoder::new(&mut arienc).unwrap();
    let decoded = decoder.collect::<Vec<u8>>();

    assert_eq!(normal, decoded);
}

#[test]
fn test_routines() {
    let file = std::fs::File::open("./war_and_peace.txt").unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let mut encoded = Vec::new();

    encode_routine(&mut bufreader, &mut encoded).unwrap();

    bufreader.rewind().unwrap();
    let mut normal = Vec::new();
    bufreader.read_to_end(&mut normal).unwrap();
    assert_ne!(normal, encoded);

    let mut decoded = Vec::new();
    decode_routine(&mut encoded.as_slice(), &mut decoded).unwrap();

    assert_eq!(normal, decoded);
}
#[test]
fn test_iterators_and_routines_equivalent() {
    let file = std::fs::File::open("./war_and_peace.txt").unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let mut encoded = Vec::new();

    encode_routine(&mut bufreader, &mut encoded).unwrap();

    bufreader.rewind().unwrap();

    let arienc = ArithmeticStreamEncoder::new(&mut bufreader)
        .unwrap()
        .collect::<Vec<u8>>();

    assert_eq!(arienc, encoded);
}



