use std::io::Write;
use std::path::Path;
use std::{fs, process};

use comrak::{markdown_to_html, ComrakOptions};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::cleanup_and_exit;

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct Frontmatter {
    title: String,
    template: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
struct FileData {
    front_matter: Frontmatter,
    content: String,
}

// @TODO cleanup/teardown function to exit the cli without partially built directory
// @TODO Error if the Front matter Cannot be converted to struct
fn get_structured_content(raw_text: &String, is_nested_dir: bool) -> Option<FileData> {
    if !raw_text.is_empty() {
        //split string on the delimiter ++++
        let split_text: Vec<&str> = raw_text.split("++++").collect();

        //use serde and toml to create Frontmatter struct
        let fm: Frontmatter = toml::from_str(split_text[0]).unwrap();

        // comrak for markdown to hmtl
        let mut options = ComrakOptions::default();
        options.render.escape = false;

        let md_html = markdown_to_html(split_text[1].trim(), &options);

        // if we are not in a nested directory template is required
        if !is_nested_dir && fm.template.is_none() {
            eprintln!("Top level data directory files require a template in the front matter.");
            cleanup_and_exit();
        }

        Some(FileData {
            front_matter: fm,
            content: md_html,
        })
    } else {
        None
    }
}

// build data files gets a path, if a dir loop if a file return contents
pub fn build_data_files(current_dir: &Path, nested_dir: bool, tera: &Tera) {
    // error check this read_dir result?
    let paths = fs::read_dir(current_dir).unwrap();

    for path in paths {
        let content_path = path
            .as_ref()
            .unwrap_or_else(|e| panic!("error for path: {:?} \n {:?}", path, e));

        if content_path.path().is_file() {
            // get the raw markdown
            let raw_text = fs::read_to_string(content_path.path());
            // get the structured content with markdown converted to html
            let structured_content = get_structured_content(&raw_text.unwrap(), nested_dir);

            if let Some(content) = structured_content {
                // render at this point
                if let Some(tmp) = &content.front_matter.template {
                    let rendered_content = tera
                        .render(tmp, &Context::from_serialize(&content).unwrap())
                        .unwrap();

                    let build_path = &content_path
                        .path()
                        .to_string_lossy()
                        .replace("/data", "/build");
                    let build_path = build_path.replace(".md", ".html");

                    create_build_path(Path::new(&build_path));

                    let mut new_file = fs::File::create(build_path).unwrap();
                    new_file.write_all(rendered_content.as_bytes()).unwrap();
                }
            }
        } else {
            build_data_files(&content_path.path(), true, tera)
        }
    }
}

fn create_build_path(build_path: &Path) {
    let mut b = build_path.to_path_buf();
    b.pop();
    println!("creating dir: {:?}", b);

    fs::create_dir_all(b).unwrap();
}

// build calls the build_data_files which loops and calls get_structured_content, then with the
// result build_data_files calls the render method
pub fn build(path: &Path) {
    let data_dir_path = path.join("data");

    // tera gets all the templates find a good spot for this
    let template_dir_path = "templates/**/*.html";
    let tera = match Tera::new(template_dir_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    // loop over data dir markdown files and structure content
    if data_dir_path.is_dir() {
        build_data_files(&data_dir_path, false, &tera)
    } else {
        eprintln!("Cannot find data directory, make sure this is a pebble project");
        process::exit(0)
    }
}
