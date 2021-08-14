use std::fs;
use std::path::Path;

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
            result.push(del_http(l[l.find('\"').unwrap()+1..l.find("\" A").unwrap()].to_string()));
        }
    }
    result
}

fn get_from_folder(folder_path: &Path) -> Vec<String> {
    let result = fs::read_dir(folder_path).unwrap().map(|file| {
        fs::read_to_string(file.unwrap().path())
        .unwrap()
        .lines()
        .map(|i| del_http(i.to_string()))
        .collect::<Vec<_>>()
    }).collect::<Vec<_>>().concat();
    
    result
}

// ? &[] or &Vec<String>
fn del_existing(all_urls: &[String], test_urls: &[String], format: &str) -> Vec<String> {
    let mut result = Vec::new();
    let step = match format {
        "new" => 2, // check if number of lines is even
        "old" => 1,
        _ => panic!("Invalid format")
    };
    for url in test_urls.iter().skip(step-1).step_by(step) {
        if !all_urls.contains(&del_http(url.to_string()))
        && !result.contains(url) {
            result.push(url.to_string());  // ? or .clone()
        }
    }

    result
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let format = &args[1];
    let source_path = Path::new(&args[2]);
    let source = fs::read_to_string(source_path).unwrap().lines().map(String::from).collect::<Vec<_>>();

    let all_urls = [
    ].concat();

    let result = del_existing(&all_urls, &source, format);
    println!("{} links are not in bookmarks", result.len());

    if result.len() > 0 {
        fs::write(
            source_path.parent().unwrap().join(
                source_path.file_stem().unwrap().to_str().unwrap().to_string() + "_uniq_rusted.txt"
            ), result.join("\n")).expect("Unable write to output file");
    }
}
