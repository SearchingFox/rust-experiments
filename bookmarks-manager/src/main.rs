use std::fs;
use std::path::Path;

fn del_http(url: &str) -> String {
    if url.starts_with("http") {
        match url.splitn(2, "://").nth(1) {
            None => {}
            Some(new_url) => return new_url.to_string(),
        }
    }

    url.to_string()
}

fn get_from_html(file_path: &Path) -> Vec<String> {
    let result = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .flat_map(|line| match line.find("\" A") {
            Some(end) => Some(del_http(&line[line.find('\"').unwrap() + 1..end])),
            None => None,
        })
        .collect();

    result
}

fn get_from_folder(folder_path: &Path) -> Vec<String> {
    let result = fs::read_dir(folder_path)
        .unwrap()
        .map(|file| {
            fs::read_to_string(file.unwrap().path())
                .unwrap()
                .lines()
                .map(|i| del_http(i))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat();

    result
}

fn del_existing(all_urls: &[String], test_urls: &[String], format: &str) -> Vec<String> {
    let step = match format {
        "new" => 2,
        "old" => 1,
        _ => panic!("Invalid format"),
    };
    let result = test_urls
        .iter()
        .skip(step - 1)
        .step_by(step)
        .flat_map(|url| {
            if !all_urls.contains(&del_http(url)) {
                Some(url.to_string())
            } else {
                None
            }
        })
        .collect();

    result
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let format = &args[1];
    let source_path = Path::new(&args[2]);
    let source = fs::read_to_string(source_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let all_urls = [
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/tabs")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/Bookmarks")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/hn_old")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/HN/18")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/HN/19")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/HN/20")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/BOOKMARKS/HN/21")),
        get_from_folder(Path::new("C:/Users/Asus/Desktop/firefox_resolve/ToSort")),
        get_from_html(Path::new(
            "C:/Users/Asus/Desktop/BOOKMARKS/bookmarks_firefox_210115_0515_noicons_rusted.html",
        )),
    ]
    .concat();

    let result = del_existing(&all_urls, &source, format);
    println!("{} links are not in bookmarks", result.len());

    if result.len() > 0 && result.len() < source.len() {
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
