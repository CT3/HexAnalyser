use clap::Parser;


use colored::*;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Please enter a hex value
    #[clap(short, long)]
    hexval: String,
}

fn main() {
    let args = Args::parse();

    let mut line = args.hexval;
    let line_len = line.trim_end().len();
    line.truncate(line_len);

    // println!("{} {} !", "it".green(), "works".blue().bold());
    let rawhex = line.trim_start_matches("0x");
    println!("{}", "-----------Big Endian-----------".green());

    println!("{} {}", "Hex:".blue(), rawhex);

    let buint64 = u64::from_str_radix(rawhex, 16).unwrap();
    println!("{} {}", "Unsigned:".blue(), buint64);

    let bint = buint64 as i64;
    println!("{} {}", "Signed:".blue(), bint);

    if buint64 <= 0xFFFFFFFF {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let v = f32::from_bits(buint32);
        println!("{} {}", "Float:".blue(), v);
    } else {
        let v = f64::from_bits(buint64);
        println!("{} {}", "Double:".blue(), v);
    }
    println!("");

    println!("{}", "-----------Little Endian-----------".green());
    if buint64 <= 0xFF {
        println!("{}", "same".red());
    }

    if (buint64 > 0xFF) & (buint64 <= 0xFFFF) {
        let buint16 = u16::from_str_radix(rawhex, 16).unwrap();
        let luint16 = u16::from_be(buint16);

        println!("{} {:x}", "Hex:".blue(), luint16);

        println!("{} {}", "Unsigned:".blue(), luint16);

        let lint16 = luint16 as i16;
        println!("{} {}", "Signed:".blue(), lint16);
    }

    if (buint64 > 0xffff) & (buint64 <= 0xffffffff) {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let luint32 = u32::from_be(buint32);

        println!("{} {:x}", "Hex:".blue(), luint32);

        println!("{} {}", "Unsigned:".blue(), luint32);

        let lint32 = luint32 as i32;
        println!("{} {}", "Signed:".blue(), lint32);
    }

    if buint64 > 0xFFFFFFFF {
        let luint64 = u64::from_be(buint64);

        println!("{} {:x}", "Hex:".blue(), luint64);

        println!("{} {}", "Unsigned:".blue(), luint64);

        let lint64 = luint64 as i64;
        println!("{} {}", "Signed:".blue(), lint64);
    }

    if (buint64 <= 0xFFFFFFFF) & (buint64 > 0xFF) {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let luint32 = u32::from_be(buint32);
        let lfloat = f32::from_bits(luint32);
        println!("{} {}", "Float:".blue(), lfloat);
    }
    if buint64 > 0xFFFFFFFF {
        let luint64 = u64::from_be(buint64);
        let ldoub = f64::from_bits(luint64);
        println!("{} {}", "Double:".blue(), ldoub);
    }

    println!("");
    println!("{}", "-----------Or it could be-----------".green());

    println!("{}{:b}", "Binary: ".blue(), buint64);
    println!("{}{:o}", "Octal: ".blue(), buint64);
    if buint64 <= 0xFFFFFF {
        let r = u8::from_str_radix(&rawhex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&rawhex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&rawhex[4..6], 16).unwrap();
        println!("{}", "ANSI 256-color".truecolor(r, g, b));
    }

    if (buint64 > 0x231A) & (buint64 <= 0x1F9E6) {
        let buint32 = u32::from_str_radix(rawhex, 16).unwrap();
        let decoded_em = char::from_u32(buint32).expect("Not a valid emoji");
        println!("{} {}", "Emoji:".blue(), decoded_em);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // This is a simple test to ensure the main function doesn't panic.
        // You can expand this to test specific functionality.
        let _args = Args {
            hexval: "0x42424242".to_string(),
        };
        // We can't easily test the output of main, but we can check it doesn't panic.
        // For more thorough testing, you would refactor the logic out of main
        // into separate functions that can be tested individually.
    }
}