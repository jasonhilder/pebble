use std::{fs, process};
use std::path::Path;

#[derive(Debug)]
struct Frontmatter {
    title: String,
    template: String
}

#[derive(Debug)]
struct FileData {
    front_matter: Frontmatter,
    md: String
}

fn get_structured_content(content: &String) -> Option<FileData> {
    if content.len() > 0 {
        //println!("contents: {:?}", content);
        // lets get the front matter
        let mut seq = String::new();
        let mut front_matter = String::new();

        for (i, c) in content.chars().enumerate() {
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

        //@todo use serde and toml to create Frontmatter struct

        Some(FileData{
            front_matter: Frontmatter{ title: String::from("test"), template: String::from("test") },
            // +3 is for the +++ we ignore
            md: content[front_matter.len()+3..].trim_start().to_string()
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

    if data_dir.is_dir() {
        // error check this read_dir result?
        let paths = fs::read_dir(data_dir).unwrap();

        for path in paths {
            // get the raw markdown
            let markdown = fs::read_to_string(path.unwrap().path());
            let sc = get_structured_content(&markdown.unwrap());

            if sc.is_some() {
                println!("{:?}", sc);
            }
        }
    } else {
        eprintln!("Cannot find data directory, make sure this is a pebble project");
        process::exit(0)
    }

}
/*
 - Loop over data dir, create structured data with FileData.
 - Place in array of FileData (for async later)
 - Loop over array and generate an actual html page using it.
*/
