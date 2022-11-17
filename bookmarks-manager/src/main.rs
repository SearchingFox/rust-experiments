#![feature(path_file_prefix)]
#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;
use std::path::Path;

use serde::Deserialize;
use serde_json::from_str;

#[derive(Debug, Eq, Hash, Clone, Deserialize)]
struct Bookmark {
    title: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct JsonFile {
    windows: HashMap<String, HashMap<String, Bookmark>>,
}

type Bookmarks = HashSet<Bookmark>;

impl Bookmark {
    fn to_string(&self, full_fmt: bool) -> String {
        if full_fmt {
            [self.title.to_string(), self.url.to_string()].join("\n")
        } else {
            self.url.to_string()
        }
    }

    fn get_url_without_http(&self) -> String {
        match self.url.split_once("://").map(|x| x.1) {
            None => self.url.to_string(),
            Some(s) => s.to_string(),
        }
    }
}

impl PartialEq for Bookmark {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

impl PartialOrd for Bookmark {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.url.cmp(&other.url))
    }
}

impl Ord for Bookmark {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.url.cmp(&other.url)
    }
}

fn write_to_file(file_name: &Path, content: &Bookmarks) -> Result<(), std::io::Error> {
    fs::write(
        file_name,
        content
            .into_iter()
            .map(|b| b.to_string(true))
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

fn get_from_txt(file_path: &Path) -> Bookmarks {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .array_chunks()
        .map(|x: [&str; 2]| Bookmark {
            title: x[0].to_string(),
            url: x[1].to_string(),
        })
        .collect()
}

fn get_from_html(file_path: &Path) -> Bookmarks {
    fs::read_to_string(file_path)
        .expect("Couldn't read html file")
        .lines()
        .filter_map(|line| {
            line.trim_start().starts_with("<DT><A").then(|| Bookmark {
                title: line[line.find("\">").unwrap() + 2..line.find("</").unwrap()].to_string(),
                url: line[line.find("HREF=\"").unwrap() + 6..line.find("\" ").unwrap()].to_string(),
            })
        })
        .collect()
}

fn get_from_md(file_path: &Path) -> Bookmarks {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|x| {
            let splitp = x.rfind("]").unwrap();
            Bookmark {
                title: x[x.find("[").unwrap() + 1..splitp].to_string(),
                url: x[splitp + 2..x.rfind(")").unwrap()].to_string(),
            }
        })
        .collect()
}

fn get_from_json(file_path: &Path) -> Bookmarks {
    let res: Vec<JsonFile> =
        from_str(&fs::read_to_string(file_path).expect("Couldn't read json file"))
            .expect("Couldn't parse json file");
    res.into_iter()
        .flat_map(|x| x.windows.into_values().flat_map(|y| y.into_values()))
        .collect()
}

fn get_from_folder(folder_path: &Path) -> Bookmarks {
    fs::read_dir(folder_path)
        .unwrap()
        .flat_map(|file| get_from_file(&file.unwrap().path()))
        .collect()
}

fn get_from_file(file_path: &Path) -> Bookmarks {
    match file_path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("md") => get_from_txt(file_path), // this is intentional
        Some("txt") => get_from_txt(file_path),
        Some("html") => get_from_html(file_path),
        Some("json") => get_from_json(file_path),
        _ => HashSet::new(),
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    match args[1].as_str() {
        "--htt" => {
            for i in &args[2..] {
                write_to_file(
                    &Path::new(i).with_extension("md"),
                    &get_from_file(Path::new(i)),
                )
                .expect("Couldn't write output file");
            }
        }
        "--uni" => {
            write_to_file(
                &Path::new(&args[2]).with_file_name("united_rusted.md"),
                &args[2..].into_iter().fold(HashSet::new(), |acc, hs| {
                    acc.union(&get_from_file(Path::new(&hs))).cloned().collect()
                }),
            )
            .expect("Couldn't write output file");
        }
        "--spt" => {
            let file_path = Path::new(&args[2]);
            let chunk_size: usize = args[3]
                .trim_end()
                .parse()
                .expect("Couldn't parse chunk size");
            let mut inp: Vec<Bookmark> = get_from_file(file_path).into_iter().collect();
            inp.sort();
            let old_file_name = file_path
                .file_prefix()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            for (i, el) in inp.chunks(chunk_size).enumerate() {
                fs::write(
                    file_path.with_file_name(old_file_name + "_" + &(i + 1).to_string() + ".md"),
                    el.into_iter()
                        .map(|x| x.to_string(true))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("Couldn't write output file");
            }
        }
        "--ddp" => {
            let source_path = Path::new(&args[2]);
            let source = get_from_file(source_path);
            let l = source.len();

            let mut all_urls: Bookmarks = HashSet::new();
            for folder_path in &args[3..] {
                all_urls.extend(get_from_folder(Path::new(folder_path)));
            }

            let result: Bookmarks = source.difference(&all_urls).cloned().collect();
            println!("{} links are not in bookmarks", result.len());

            if !result.is_empty() && result.len() < l {
                fs::write(
                    source_path.with_file_name(
                        source_path
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            + "_uniq_rusted.md",
                    ),
                    result
                        .into_iter()
                        .map(|x| x.to_string(true))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("Write to output file failed");
            }
        }
        _ => {}
    }
}
