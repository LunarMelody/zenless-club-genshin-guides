use pulldown_cmark::Options;
use pulldown_cmark::{html::push_html, Parser as CmarkParser};
use std::error::Error;
use std::{fs, path::PathBuf};

const PARSER_OPTIONS: Options = Options::all();

pub fn md_to_html(source: String) -> String {
    let parser = CmarkParser::new_ext(&source, PARSER_OPTIONS);

    let mut html = String::new();
    push_html(&mut html, parser);

    html
}

pub fn md_file_to_html(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;

    Ok(md_to_html(content))
}

pub fn make_output_path(dir: &str, file_name: &str) -> PathBuf {
    let path: PathBuf = [dir, file_name].iter().collect();

    path
}

pub fn make_output_file(path: PathBuf) -> Result<fs::File, Box<dyn Error>> {
    let parent_dir = path.parent().unwrap();

    if !parent_dir.exists() {
        fs::create_dir_all(parent_dir)?
    }

    let file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?;

    Ok(file)
}
