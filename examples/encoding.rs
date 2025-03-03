
use simple_arithmetic_coding::{ArithmeticStreamDecoder, ArithmeticStreamEncoder};


fn safely_encode(v: &[u64]) -> Vec<u8>
{
    let nums = v.iter()
        .flat_map(|e| e.to_ne_bytes())
        .collect::<Vec<u8>>();

    ArithmeticStreamEncoder::new(&mut &nums[..])
        .unwrap()
        .collect()
}

unsafe fn encode(v: &[u64]) -> Vec<u8>
{
    let (_, mut slice, _) = v.align_to::<u8>();

    ArithmeticStreamEncoder::new(&mut slice)
        .unwrap()
        .collect()
}

unsafe fn decode(mut v: &[u8]) -> Vec<u64>
{
    let o = ArithmeticStreamDecoder::new(&mut v)
        .unwrap()
        .collect::<Vec<u8>>();

    let (_, slice, _) = o.align_to::<u64>();

    Vec::from(slice)
}

fn main() {
    let data = vec![4,3,2,1,10,1,1,0,0,0];
    println!(
        "Encoding [{}]...",
        data.iter()
            .map(|t| format!("{:064b}", t))
            .reduce(|acc,e| acc+e.as_str())
            .unwrap()
    );

    let enc_safe = safely_encode(&data);
    let enc_not_safe = unsafe { encode(&data) };

    assert_eq!(enc_safe, enc_not_safe);

    println!();
    println!(
        "Result: {}",
        enc_not_safe.iter()
            .map(|t| format!("{:008b}", t))
            .reduce(|acc,e| acc+e.as_str())
            .unwrap()
    );

    println!();

    println!("Decoding back...");

    let decode_not_safe = unsafe { decode(&enc_not_safe) };

    println!(
        "Result: [{}]",
        decode_not_safe.iter()
            .map(|t| format!("{:064b}", t))
            .reduce(|acc,e| acc+e.as_str())
            .unwrap()
    );

    assert_eq!(decode_not_safe, data);

    println!("Success");
}
