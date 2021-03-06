use anyhow::{Context, Result};
use image::GenericImageView;
use std::io::Write;

/// Example
///  
/// $ cargo run source/member.bmp
fn main() -> Result<()> {
    if std::env::args_os().len() > 1 {
        let args: Vec<String> = std::env::args().collect();
        let filename = &args.last().unwrap();
        let file_path = std::path::Path::new(filename);
        if file_path.exists() {
            let source =
                image::open(file_path).context(format!("unable to open '{:?}'", file_path))?;
            let (width, height) = source.dimensions();
            let gray = source.to_luma8();
            if (width * height) % 8 != 0 {
                eprintln!("For correct use of HEX, the number of pixels must be divisible by 8");
            }
            let len: usize = ((width * height) / 8).try_into()?;
            let mut buf: Vec<String> = Vec::with_capacity(len);
            let mut one_unit = String::from("");
            for (_c, pix) in gray.to_vec().iter().enumerate() {
                let logic = match pix {
                    n if n > &1 => "0",
                    _ => "1",
                };
                one_unit.push_str(logic);
                if one_unit.len() == 8 {
                    buf.push(one_unit);
                    one_unit = "".to_owned();
                }
            }
            if one_unit.len() > 0 {
                buf.push(one_unit);
            }
            // Output STDOUT
            for (c, i) in buf.iter().enumerate() {
                if c % (width / 8) as usize == 0 {
                    println!("");
                }
                print!("{:#X},", u32::from_str_radix(i, 2)?);
            }
            // Output FILE
            let out = std::fs::File::create("source/output_hex")?;
            let mut buf_out = std::io::BufWriter::new(out);
            for (c, i) in buf.iter().enumerate() {
                if c % (width / 8) as usize == 0 {
                    let _ = writeln!(buf_out, "");
                }
                write!(buf_out, "{:#X},", u32::from_str_radix(i, 2)?)?;
            }
        } else {
            eprintln!("Error:\n File {:?} not found in this scope", filename);
        }
    } else {
        println!("Run:\n $cargo run source/<FILE NAME BMP>");
    }
    Ok(())
}
