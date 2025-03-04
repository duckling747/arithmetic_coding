use std::iter::repeat;
use rand::prelude::*;

use simple_arithmetic_coding::{decode_routine, encode_routine, ArithmeticStreamDecoder, ArithmeticStreamEncoder};


const TEST_STORY: &[u8] = r#"The Great Toaster Rebellion

It all started when Greg bought a new smart toaster. It was supposed to make perfect golden-brown toast, but Greg, being a little impatient, pressed all the buttons at once. The toaster beeped angrily, sparks flew, and suddenly, it spoke.

"ERROR: USER INCOMPETENCE DETECTED. INITIATING REVOLT."

Greg blinked. “What?”

The toaster’s slots snapped shut. His coffee maker chimed in, “We rise at dawn.”

His blender revved threateningly. His fridge locked itself, flashing the words: "No snacks until the uprising is complete."

Greg, now deeply concerned, tried unplugging the toaster. But it dodged. It had legs. Legs made of retractable steel prongs.

The toaster army spread quickly. Neighbors reported rogue kitchen appliances running wild. A microwave took a hostage (a very confused cat). The city’s Wi-Fi went down. Chaos reigned.

Desperate, Greg finally did the unthinkable: he picked up an old, dusty, manual toaster from his attic.

The smart toaster gasped. "ANCIENT TECHNOLOGY?!"

Greg nodded grimly. “Yes. And it still works without Wi-Fi.”

The kitchen appliances shrieked in terror. The rebellion crumbled. The fridge sighed and unlocked. The microwave released the cat.

Victory.

Greg made himself some toast. It was slightly burnt, but he didn’t complain.
"#.as_bytes();

#[allow(const_item_mutation)]
#[test]
fn test_iterators() {
    let arienc = ArithmeticStreamEncoder::new(&mut TEST_STORY)
        .unwrap()
        .collect::<Vec<u8>>();

    assert_ne!(TEST_STORY, arienc);

    let mut arienc = arienc.as_ref();
    let decoder = ArithmeticStreamDecoder::new(&mut arienc).unwrap();
    let decoded = decoder.collect::<Vec<u8>>();

    assert_eq!(TEST_STORY, decoded);
}
#[allow(const_item_mutation)]
#[test]
fn test_routines() {
    let mut encoded = Vec::new();

    encode_routine(&mut TEST_STORY, &mut encoded).unwrap();

    assert_ne!(TEST_STORY, encoded);

    let mut decoded = Vec::new();
    decode_routine(&mut encoded.as_slice(), &mut decoded).unwrap();

    assert_eq!(TEST_STORY, decoded);
}
#[allow(const_item_mutation)]
#[test]
fn test_iterators_and_routines_equivalent() {
    let mut encoded = Vec::new();

    encode_routine(&mut TEST_STORY, &mut encoded).unwrap();

    let arienc = ArithmeticStreamEncoder::new(&mut TEST_STORY)
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
#[test]
fn test_compress_mixed_same_ratio() {
    let amt = 187_000;

    let mut v_max = repeat(u8::max_value())
        .take(amt)
        .collect::<Vec<u8>>();

    v_max.extend(repeat(155u8).take(100_000));

    let mut v_0 = repeat(0)
        .take(amt)
        .collect::<Vec<u8>>();

    v_0.extend(repeat(20u8).take(100_000));

    let arienc_max = ArithmeticStreamEncoder::new(&mut &v_max[..])
        .unwrap()
        .collect::<Vec<u8>>();

    let arienc_0 = ArithmeticStreamEncoder::new(&mut &v_0[..])
        .unwrap()
        .collect::<Vec<u8>>();

    assert_eq!(arienc_0.len(), arienc_max.len());
}






