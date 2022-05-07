fn main() {
    let mut line = String::new();

    println!("Enter Hex:");
    let _b1 = std::io::stdin().read_line(&mut line).unwrap();
    let line_len = line.trim_end().len();
    line.truncate(line_len);

    let rawhex = line.trim_start_matches("0x");
    println!("-----------Big Endian-----------");

    println!("Hex: {}", rawhex);

    let buint64 = u64::from_str_radix(rawhex, 16).unwrap();
    println!("Positive: {}", buint64);

    let bint = unsafe { std::mem::transmute::<u64, i64>(buint64) };
    println!("Negative: {}", bint);

    if buint64 <= 0xFFFFFFFF {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let v = f32::from_bits(buint32);
        println!("Float: {}", v);
    } else {
        let v = f64::from_bits(buint64);
        println!("double: {}", v);
    }

    println!("-----------Little Endian-----------");
    if buint64 <= 0xFF {
        println!("same");
    }

    if (buint64 > 0xFF) & (buint64 <= 0xFFFF) {
        let buint16 = u16::from_str_radix(rawhex, 16).unwrap();
        let luint16 = u16::from_be(buint16);

        println!("Hex: {:x}", luint16);

        println!("Positive: {}", luint16);

        let lint16 = unsafe { std::mem::transmute::<u16, i16>(luint16) };
        println!("Negative: {}", lint16);
    }

    if (buint64 > 0xFFFF) & (buint64 <= 0xFFFFFFFF) {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let luint32 = u32::from_be(buint32);

        println!("Hex: {:x}", luint32);

        println!("Positive: {}", luint32);

        let lint32 = unsafe { std::mem::transmute::<u32, i32>(luint32) };
        println!("Negative: {}", lint32);
    }

    if buint64 > 0xFFFFFFFF {
        let luint64 = u64::from_be(buint64);

        println!("Hex: {:x}", luint64);

        println!("Positive: {}", luint64);

        let lint64 = unsafe { std::mem::transmute::<u64, i64>(luint64) };
        println!("Negative: {}", lint64);
    }

    if (buint64 <= 0xFFFFFFFF) && (buint64 > 0xFF) {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let luint32 = u32::from_be(buint32);
        let lfloat = f32::from_bits(luint32);
        println!("Float: {}", lfloat);
    }
    if buint64 > 0xFFFFFFFF {
        let luint64 = u64::from_be(buint64);
        let ldoub = f64::from_bits(luint64);
        println!("Double: {}", ldoub);
    }

    // let _r1 = std::io::stdin().read_line(&mut line).unwrap();
}
