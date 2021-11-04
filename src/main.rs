
use std::io::Write; 
use image::GenericImageView;

/// Dependencies
/// 
/// [dependencies]
/// image = "0.23"

/// Example
///  
/// $ cargo run ../member.bmp
fn main() {
    if std::env::args_os().len() >1 {
        let args: Vec<String> = std::env::args().collect();
        let filename = &args.last().unwrap();
        let file_path =  std::path::Path::new(filename);
        if file_path.exists() {  
            let source = image::open(file_path).unwrap();
            let (width, height) = source.dimensions();
            let gray = source.to_luma();
            let len:usize = ((width*height)/8).try_into().unwrap();
            let mut buf: Vec<String> = Vec::with_capacity(len);
            let mut one_unit = String::from("");
            for (c,pix) in gray.to_vec().iter().enumerate(){    
                let logic= match pix{
                    n if n > &1 => "0",
                    _ => "1"
                };
                one_unit.push_str(logic);
                if one_unit.len() == 8{
                    buf.push(one_unit);
                    one_unit = "".to_owned(); 
                }
            }
            if one_unit.len() > 0{
                buf.push(one_unit);
            
            }
            // Output STDOUT
            for (c,i) in buf.iter().enumerate(){
                if c%(width/8) as usize==0 {
                    println!("");
                }
                print!("{:#X},", u32::from_str_radix(i, 2).unwrap());
            }
            // Output FILE
            let mut out = std::fs::File::create("../output_hex").unwrap();
            let mut buf_out = std::io::BufWriter::new(out);
            for (c,i) in buf.iter().enumerate(){
                if c%(width/8) as usize==0 {
                    writeln!(buf_out,"");
                }
                write!(buf_out, "{:#X},", u32::from_str_radix(i, 2).unwrap()).unwrap();
            }
        }else{
            println!("Warning:\n File {:?} not found in this scope",filename);
        }
    }else{
        println!("Run:\n $cargo run <PATH FILE>");
    }
}
 