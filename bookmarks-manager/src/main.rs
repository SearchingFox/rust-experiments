use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

#[derive(Debug, Eq)]
struct Bookmark {
    title: String,
    url: String,
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

impl Hash for Bookmark {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.url.hash(state)
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

fn get_from_file(file_path: &Path) -> Bookmarks {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|x| Bookmark {
            title: x[0].to_string(),
            url: x[1].to_string(),
        })
        .collect()
}

fn get_from_html(file_path: &Path) -> Bookmarks {
    fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .filter_map(|line| {
            if line.trim_start().starts_with("<DT><A") {
                let title =
                    line[line.find("\">").unwrap() + 2..line.find("</").unwrap()].to_string();
                let url =
                    line[line.find("HREF=\"").unwrap() + 6..line.find("\" ").unwrap()].to_string();
                Some(Bookmark { title, url })
            } else {
                None
            }
        })
        .collect()
}

fn get_from_md(inp: String) -> Bookmarks {
    inp.lines()
        .map(|x| {
            let splitp = x.rfind("]").unwrap();
            let title = x[x.find("[").unwrap() + 1..splitp].to_string();
            let url = x[splitp + 2..x.rfind(")").unwrap()].to_string();
            Bookmark { title, url }
        })
        .collect()
}

fn get_from_folder(folder_path: &Path) -> Bookmarks {
    fs::read_dir(folder_path)
        .unwrap()
        .flat_map(|file| get_from_file(&file.unwrap().path()))
        .collect()
}

fn del_existing(all_urls: Bookmarks, test_urls: Bookmarks) -> Bookmarks {
    test_urls
        .iter()
        .flat_map(|&b| all_urls.iter().find(|&x| x.url == b.url).map_or(Some(b), |_| None))
        .collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let cmd = args[1].as_str();
    match cmd {
        "--htt" => {
            for i in &args[2..] {
                let p = Path::new(i);
                fs::write(
                    p.with_extension("txt"),
                    get_from_html(&p)
                        .into_iter()
                        .map(|x| x.to_string(true))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("Couldn't write output file");
            }
        }
        "--uni" => {
            let mut res: HashSet<Bookmark> = HashSet::new();
            for i in &args[2..] {
                res.extend(get_from_html(Path::new(&i)));
            }
            fs::write(
                "uni.txt",
                res.into_iter()
                    .map(|x| x.to_string(true))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .expect("Couldn't write output file");
        }
        "--spt" => {
            let file_path = Path::new(&args[2]);
            let chunk_size: usize = args[3].trim_end().parse().expect("Couldn't parse chunk size");
            let mut inp = get_from_file(file_path).into_iter().collect::<Vec<_>>();
            inp.sort();

            for (i, el) in inp.chunks(chunk_size).enumerate() {
                fs::write(
                    file_path.file_stem().unwrap().to_str().unwrap().to_string()
                        + "_"
                        + &(i + 1).to_string()
                        + ".md",
                    el.into_iter()
                        .map(|x| x.to_string(true))
                        .collect::<Vec<_>>()
                        .join("\n"),
                )
                .expect("Couldn't write output file");
            }
        }
        _ => {
            let format = &args[2];
            let source_path = Path::new(&args[3]);
            let source = get_from_file(source_path);

            let all_urls = HashSet::new();

            let result = del_existing(all_urls, source);
            println!("{} links are not in bookmarks", result.len());

            if !result.is_empty() && result.len() < source.len() {
                fs::write(
                    source_path.parent().unwrap().join(
                        source_path
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string()
                            + "_uniq_rusted.txt",
                    ),
                    result.join("\n"),
                )
                .expect("Write to output file failed");
            }
        }
    }
}
