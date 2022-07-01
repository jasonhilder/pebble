use std::{fs, process};
use std::path::Path;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: String,
    template: Option<String>
}

#[allow(dead_code)]
#[derive(Debug)]
struct FileData {
    front_matter: Frontmatter,
    md: String
}

// @TODO cleanup/teardown function to exit the cli without partially built directory

// @TODO Error if the Front matter Cannot be converted to struct
fn get_structured_content(raw_text: &String, is_nested_dir: bool) -> Option<FileData> {
    if !raw_text.is_empty() {
        //split string on the delimiter ++++
        let split_text: Vec<&str> = raw_text.split("++++").collect();

        //use serde and toml to create Frontmatter struct
        let fm: Frontmatter = toml::from_str(split_text[0]).unwrap();

        // if we are not in a nested directory template is required
        if !is_nested_dir && fm.template.is_none() {
            eprintln!("Top level data directory files require a template in the front matter.");
            process::exit(0)
        }

        Some(FileData{
            front_matter: fm,
            md: split_text[1].trim().to_string() // +3 is for the "+++" we ignore
        })
    } else {
        None
    }
}

// build data files gets a path, if a dir loop if a file return contents
pub fn build_data_files(current_dir: &Path, nested_dir: bool) {
        // error check this read_dir result?
        let paths = fs::read_dir(current_dir).unwrap();

        for path in paths {
            if path.as_ref().unwrap().path().is_file() {
                // get the raw markdown
                let raw_text = fs::read_to_string(path.as_ref().unwrap().path());
                let structured_content = get_structured_content(&raw_text.unwrap(), nested_dir);

                if let Some(content) = structured_content {
                    // render at this point
                    println!("path: {:?}\n", path.unwrap().path());
                    println!("{:#?}\n", content);
                }
            } else {
                build_data_files(&path.unwrap().path(), true)
            }
        }
}

// @FIXME Recursively walk the directory
// build calls the build_data_files which loops and calls get_structured_content, then with the
// result build_data_files calls the render method
pub fn build(path: &Path) {
    let data_dir_path = path.join("data");

    // loop over data dir markdown files and structure content
    if data_dir_path.is_dir() {
        build_data_files(&data_dir_path, false)
    } else {
        eprintln!("Cannot find data directory, make sure this is a pebble project");
        process::exit(0)
    }
}
/*
 * @TODO get tera rendering templates first
 - Loop over data dir, create structured data with FileData.
 - Place in array of FileData (for async later)
 - Loop over array and generate an actual html page using it.
*/
