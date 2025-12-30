use clap::{Parser};
// use image::GenericImageView;

mod io;

#[derive(Parser,Debug)]
#[command(version,about="Test test",)]
struct MandatoryArgs{
    /// Image path to read for manipulation
    #[arg(short,long)]
    input: String,

    /// Image path to save the result
    #[arg(short,long,default_value="./result.png")]
    output: String,
}


fn main() {
    let args = MandatoryArgs::parse();
    
    println!("test {}",args.input);
    println!("test {}",args.output);

    io::read_image();
    io::save_image();
}


