#![feature(try_trait)]
use std::fs;
use std::path::Path;

fn deduplicate(inp: &Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(inp.len());
    for i in inp {
        if !result.contains(i) {
            result.push(i.to_string());
        }
    }
    result
}

fn del_http(url: String) -> String {
    if url.starts_with("http") {
        let t = url.splitn(2, "://").collect::<Vec<_>>();
        if t.len() > 1 {
            return t[1].to_string()
        }
    }
    url
}

fn get_from_html(file_path: &Path) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for l in fs::read_to_string(file_path).unwrap().lines() {
        if l.contains("<A") {
            result.push(del_http(l[l.find("\"").unwrap()+1..l.find("\" A").unwrap()].to_string()));
        }
    }
    result
}

fn get_from_folder(folder_path: &Path) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for file in fs::read_dir(folder_path).unwrap() {
        result.extend( // ? append
            fs::read_to_string(file.unwrap().path())
            .unwrap()
            .split("\n")
            // .filter(|i| i.starts_with("http"))
            .map(|i| del_http(i.to_string()))
            .collect::<Vec<_>>());
    }
    deduplicate(&result)
}

fn del_existing(all_urls: &Vec<String>, test_urls: &Vec<String>, format: &str) -> Vec<String> {
    let mut result = Vec::new();
    if format == "new" {
        for (i, j) in (&test_urls).iter().enumerate() {
            if i % 2 == 1 && !all_urls.contains(&del_http(j.to_string())) {
                result.push(j.to_string());  // ? or .clone()
            }
        }
    } else if format == "old" {
        for i in test_urls {
            if !all_urls.contains(&del_http(i.to_string())) {
                result.push(i.to_string());
            }
        }
    }
    deduplicate(&result)
}

fn main() -> Result<(), Box<std::option::NoneError>> {
    // let arg = std::env::args().nth(1).expect("no file given");
    // let source_path = Path::new(&arg);
    let source_path = Path::new(r#""#);
    let source = fs::read_to_string(source_path).unwrap().split("\n").map(String::from).collect::<Vec<_>>();

    let bookmarks_path = Path::new(r#"C:\Users\Asus\Desktop\bookmarks_firefox_200723_0329_noicons.html"#);
    let mut all_urls = get_from_folder(Path::new(r#"C:\Users\Asus\Desktop\firefox_resolve\tabs"#));
    all_urls.extend(get_from_html(bookmarks_path));

    let result = del_existing(&all_urls, &source, "new");
    println!("notin bookmarks: {}", result.len());

    fs::write(
        source_path.parent()?.join(
            source_path.file_stem()?.to_str()?.to_string() + "_uniq_rusted.txt"
        ), result.join("\n")
    ).expect("Unable write to file");

    Ok(())
}