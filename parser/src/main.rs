mod model;

use model::Guide;

use clap::Parser as ClapParser;
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

    let dir =
        fs::read_dir(&args.input_dir).expect(&format!("{} dir doesn't exist!", &args.input_dir));

    for entry in dir.into_iter() {
        let path = entry.unwrap().path();
        let path_str = path.to_str().unwrap();

        let file_name = path.file_name().unwrap().to_str().unwrap();
        let file_prefix = file_name.split(".").next().unwrap();

        if path_str.ends_with(".md") {
            let html = md_file_to_html(path_str).unwrap();

            let guide = Guide::new(file_prefix.to_string(), html);
            let json_stringified = serde_json::to_string_pretty(&guide).unwrap();

            let resulting_path = output_file(file_prefix, &args.output_dir);

            println!("* writing to {:?}", &resulting_path);
            fs::write(resulting_path, json_stringified)
                .expect("! oh no, I couldn't write results to the file :(")
        }
    }

    println!("> done!");
}
