use std::fs;
use std::path::Path;
use std::collections::HashMap;

extern crate time;
// #[macro_use]
// extern crate serde_derive;

// #[derive(Debug, Deserialize)]
// struct Link {
//     title: Option<String>,
//     url:   Option<String>,
// }

// #[derive(Debug, Deserialize)]
// struct All {
//     windows: HashMap<String, HashMap<String, Link>>
// }

// fn get_from_json(path: &Path) -> Vec<String> {
//     let data = fs::read_to_string(path).unwrap();
//     let all: Vec<All> = serde_json::from_str(&data).unwrap();
//     let mut links = Vec::new();
//     for i in all {
//         for (_, window) in i.windows {
//             for (_, v) in window {
//                 match &v.url {
//                     None      => (),
//                     Some(url) => links.push(url.as_str().to_string())
//                 }
//             }
//             links.push("-".repeat(50).to_string())
//         }
//     }

//     return links;
// }

fn get_from_html(file_path: &Path) -> Vec<String> { // ?Result
    let data = fs::read_to_string(file_path).unwrap();
    let mut result: Vec<String> = Vec::new();
    for l in data.lines() {
        if l.contains("<A") {
            result.push(l[l.find("\"").unwrap()+1..l.find("\" A").unwrap()].to_string());
        }
    }

    return result;
}

fn del_existing(all: Vec<String>, test: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    for i in test {
        let j: Vec<&str> = i.split("://").collect();
        if j.len() > 1 {
            if !all.contains(&j[1].to_string()) {
                res.push(i);
            }
        }
        else {
            println!("{:?}", &j);
        }
    }

    return res;
}

fn main() -> std::io::Result<()> {
    let mut tabs: Vec<String> = Vec::new();
    for path in fs::read_dir(r#"C:\Users\Asus\Desktop\firefox_resolve\tabs"#).unwrap() {
        tabs.append(&mut fs::read_to_string(path?.path())?.split("\n").map(String::from).collect());
    }

    let source_path = Path::new(r#"C:\Users\Asus\Desktop\firefox_resolve\bookmarks_firefox_180829_2014_copy_copy.html"#);
    let source = get_from_html(source_path);
    let mut all_links = get_from_html(Path::new(r#"C:\Users\Asus\Desktop\bookmarks_firefox_191120_2052_noicons.html"#));
    all_links.append(&mut tabs);

    fs::write("C:\\Users\\Asus\\Desktop\\test.txt", del_existing(all_links, source).join("\n")).expect("Unable write to file");

    Ok(())
}