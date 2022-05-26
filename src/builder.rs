use std::{fs, process};
use std::path::Path;

use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Frontmatter {
    title: String,
    template: String
}

#[allow(dead_code)]
#[derive(Debug)]
struct FileData {
    front_matter: Frontmatter,
    md: String
}

fn get_structured_content(raw_text: &String) -> Option<FileData> {
    if raw_text.len() > 0 {
        // lets get the front matter
        let mut seq = String::new();
        let mut front_matter = String::new();

        for (i, c) in raw_text.chars().enumerate() {
            if i != 0 && i % 3 == 0 {
                if seq == "+++" {
                    break;
                } else {
                    front_matter.push_str(&seq);
                    // reset seq
                    seq = String::new();
                }
            }
            seq.push(c);
        }

        //use serde and toml to create Frontmatter struct
        let fm: Frontmatter = toml::from_str(&front_matter).unwrap();

        Some(FileData{
            front_matter: fm,
            md: raw_text[front_matter.len()+3..].trim_start().to_string() // +3 is for the "+++" we ignore
        })
    } else {
        None
    }
}

pub fn build(path: &Path) {
    let data_dir = path.join("data");
    // println!("Build the site");
    // println!("path: {:?}", path);
    // println!("data_dir: {:?}", data_dir);

    // array of FileData to be built
    let mut build_list: Vec<FileData> = Vec::new();

    // loop over data dir markdown files and structure content
    if data_dir.is_dir() {
        // error check this read_dir result?
        let paths = fs::read_dir(data_dir).unwrap();

        for path in paths {
            // get the raw markdown
            let raw_text = fs::read_to_string(path.unwrap().path());
            let structured_content = get_structured_content(&raw_text.unwrap());

            if structured_content.is_some() {
                build_list.push(structured_content.unwrap())
            }
        }

        println!("{:#?}", build_list);
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
