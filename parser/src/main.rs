mod model;

use model::Guide;

use clap::Parser as ClapParser;
use glob::glob;
use pulldown_cmark::{html::push_html, Parser as CmarkParser};
use std::error::Error;
use std::{fs, path::PathBuf};

#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    input_dir: String,

    #[clap(short, long, default_value = "./output")]
    output_dir: String,
}

fn md_to_html(source: String) -> String {
    let parser = CmarkParser::new(&source);

    let mut html = String::new();
    push_html(&mut html, parser);

    html
}

fn md_file_to_html(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    Ok(md_to_html(content))
}

fn output_path_buf(path: &str) -> PathBuf {
    let resulting_dir = PathBuf::from(&path);
    if !resulting_dir.exists() {
        println!("> {:?} doesn't exist :o", &resulting_dir);
        println!("> no worries, I'll make one! :)");
        fs::create_dir_all(&resulting_dir).expect("and I failed :(");
    }

    resulting_dir
}

fn output_file(file_prefix: &str, output_dir: &str) -> PathBuf {
    let resulting_dir = output_path_buf(output_dir);
    resulting_dir.join(format!("{}.json", file_prefix))
}

fn main() {
    let args = Args::parse();

    let pattern = args.input_dir + "/**/*.md";

    for entry in glob(&pattern).expect("! failed to read glob") {
        let entry = entry.unwrap();

        let file_name = entry.file_name().unwrap();
        let file_prefix = file_name
            .to_str()
            .unwrap()
            .split(".")
            .nth(0)
            .unwrap()
            .to_owned();

        let html = md_file_to_html(&entry.display().to_string()).unwrap();

        let output_path = output_file(&file_prefix, &args.output_dir);

        let guide = Guide::new(file_prefix, html);
        let jsonic = serde_json::to_string_pretty(&guide).unwrap();

        println!("* writing to {:?}...", output_path);
        fs::write(output_path, jsonic).unwrap();
    }

    println!("> done!");
}
