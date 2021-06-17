use chunk::Chunk;
use chunk_type::ChunkType;
use png::Png;
use std::{convert::TryFrom, fs::write, path::PathBuf, str::FromStr};
use structopt::StructOpt;

// mod args;
mod chunk;
mod chunk_type;
// mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, StructOpt)]
#[structopt(name = "pngme", about = "Ecode/Decode message in png file")]
enum Opt {
    Encode {
        #[structopt(parse(from_os_str), help = "Encode to png file")]
        png: PathBuf,
        #[structopt(help = "Type codes are restricted to 4 characters")]
        chunk_type: String,
        message: String,
        #[structopt(parse(from_os_str))]
        output: Option<PathBuf>,
    },
    Decode {
        #[structopt(parse(from_os_str))]
        png: PathBuf,
        chunk_type: String,
    },
    Remove {
        #[structopt(parse(from_os_str))]
        png: PathBuf,
        chunk_type: String,
    },
    #[structopt(about = "Print png chunks")]
    Print {
        #[structopt(parse(from_os_str))]
        png: PathBuf,
    },
}

fn main() -> Result<()> {
    match Opt::from_args() {
        Opt::Encode {
            png,
            chunk_type,
            message,
            output,
        } => {
            let bytes = std::fs::read(png)?;
            let mut png = Png::try_from(bytes.as_ref())?;
            let chunk_type = ChunkType::from_str(&chunk_type)?;
            let chunk = Chunk::new(chunk_type, message.as_bytes());
            png.append_chunk(chunk);
            match output {
                Some(p) => write(p, png.as_bytes())?,
                None => println!("{}", png),
            }
        }
        Opt::Decode { png, chunk_type } => {
            let bytes = std::fs::read(png)?;
            let png = Png::try_from(bytes.as_ref())?;
            match png.chunk_by_type(&chunk_type) {
                Some(chunk) => println!("{}", chunk.data_as_string()?),
                None => println!("Nothing"),
            }
        }
        Opt::Remove { png, chunk_type } => {
            let bytes = std::fs::read(&png)?;
            let mut pngme = Png::try_from(bytes.as_ref())?;
            pngme.remove_chunk(&chunk_type)?;
            write(png, pngme.as_bytes())?;
        }
        Opt::Print { png } => {
            let bytes = std::fs::read(png)?;
            let png = Png::try_from(bytes.as_ref())?;
            println!("{}", png);
        }
    }
    Ok(())
}
