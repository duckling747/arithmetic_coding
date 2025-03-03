use std::{io::{Read, Seek}, iter::repeat};
use rand::prelude::*;

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
#[test]
fn test_fuzz() {
    let rng = rand::rng();
    let v = rng.sample_iter(rand::distr::StandardUniform)
        .take(1_000_000)
        .collect::<Vec<u8>>();

    let arienc = ArithmeticStreamEncoder::new(&mut &v[..])
        .unwrap()
        .collect::<Vec<u8>>();

    let aridec = ArithmeticStreamDecoder::new(&mut &arienc[..])
        .unwrap()
        .collect::<Vec<u8>>();

    assert_eq!(aridec, v);
}
#[test]
fn test_compress_0() {
    let v = repeat(0)
        .take(10)
        .collect::<Vec<u8>>();

    let arienc = ArithmeticStreamEncoder::new(&mut &v[..])
        .unwrap()
        .collect::<Vec<u8>>();

    assert!(v.len() > arienc.len());
}
#[test]
fn test_compress_1() {
    let v = repeat(1)
        .take(10)
        .collect::<Vec<u8>>();

    let arienc = ArithmeticStreamEncoder::new(&mut &v[..])
        .unwrap()
        .collect::<Vec<u8>>();

    assert!(v.len() > arienc.len());
}
#[test]
fn test_compress_max() {
    let v = repeat(u8::max_value())
        .take(10)
        .collect::<Vec<u8>>();

    let arienc = ArithmeticStreamEncoder::new(&mut &v[..])
        .unwrap()
        .collect::<Vec<u8>>();

    assert!(v.len() > arienc.len());
}
#[test]
fn test_compress_0_max_same_ratio() {
    let amt = 187_000;

    let v_max = repeat(u8::max_value())
        .take(amt)
        .collect::<Vec<u8>>();

    let v_0 = repeat(0)
        .take(amt)
        .collect::<Vec<u8>>();

    let arienc_max = ArithmeticStreamEncoder::new(&mut &v_max[..])
        .unwrap()
        .collect::<Vec<u8>>();

    let arienc_0 = ArithmeticStreamEncoder::new(&mut &v_0[..])
        .unwrap()
        .collect::<Vec<u8>>();

    assert_eq!(arienc_0.len(), arienc_max.len());
}







