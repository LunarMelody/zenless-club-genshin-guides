mod model;
mod utils;

use model::Guide;

use clap::Parser as ClapParser;
use glob::glob;
use std::error::Error;
use std::io::Write;
use std::{fs, path::PathBuf};

use crate::utils::{md_file_to_html, make_output_path, make_output_file};

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_dir: String,

    #[clap(short, long, default_value = "./output")]
    output_dir: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut published_guides: Vec<String> = Vec::new();
    let pattern = args.input_dir + "/**/*.md";

    for entry in glob(&pattern)? {
        let entry = entry?;

        let file_name = entry.file_name().unwrap();
        let file_prefix = file_name
            .to_str()
            .unwrap()
            .split(".")
            .nth(0)
            .unwrap()
            .to_owned();

        let html = md_file_to_html(&entry.display().to_string())?;
        let guide = Guide::new(&file_prefix, &html);
        let jsonic = serde_json::to_string(&guide)?;

        let output_path = make_output_path(&args.output_dir, &format!("{}.json", &file_prefix));
        println!("* writing to {:?}...", output_path);

        let mut file = make_output_file(output_path)?;
        file.write_all(jsonic.as_bytes())?;

        published_guides.push(file_prefix);
    }

    let published_path: PathBuf = [&args.output_dir, "published.json"].iter().collect();
    println!("* writing to {:?}...", published_path);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(published_path)?;

    let jsonic = serde_json::to_string(&published_guides)?;
    file.write_all(jsonic.as_bytes())?;

    println!("> done!");
    Ok(())
}
