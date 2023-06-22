use anyhow::Result;
use clap::Parser;
use pgs_subtitles::segments::parse_segments;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    /// The directory to output the manifest and image files.
    ///
    /// By default, this is the name of the input file without an extension.
    out_dir: Option<PathBuf>,

    /// The .sup file to dump.
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse_from(wild::args());
    println!("args: {:?}", args);

    let out_dir: PathBuf;

    if let Some(out_dir_arg) = args.out_dir {
        out_dir = out_dir_arg;
    } else {
        if let (Some(parent), Some(prefix)) = (args.file.parent(), args.file.file_stem()) {
            out_dir = parent.join(prefix);
        } else {
            panic!("Couldn't determine output folder!");
        }
    }

    fs::create_dir_all(&out_dir)?;
    let raw_json_path = out_dir.join("raw_segments.json");
    let json_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(raw_json_path)?;

    let input_contents = fs::read(args.file)?;
    let segments = parse_segments(&input_contents)?;

    serde_json::to_writer(json_file, &segments.clone())?;

    Ok(())
}
