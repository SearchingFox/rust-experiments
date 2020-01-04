use std::fs;
use std::path::Path;
use std::path::PathBuf;
// use std::collections::HashMap;

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
    let mut result = Vec::new();
    for i in test {
        let j: Vec<&str> = i.split("://").collect();
        if j.len() > 1 {
            if !all.contains(&j[1].to_string()) {
                result.push(i);
            }
        }
        else {
            result.push(j[0].to_string());
        }
    }
    result.sort_unstable();
    result.dedup();
    return result;
}

fn get_from_folder(folder_path: &Path) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for path in fs::read_dir(folder_path).unwrap() {
        result.append(
            &mut fs::read_to_string(path.unwrap().path())
            .unwrap()
            .split("\n")
            .filter(|i| i.starts_with("http"))
            .map(String::from)
            .collect());
    }
    result.sort_unstable(); // use itertools::Itertools;
    result.dedup(); // let v: Vec<_> = v.into_iter().unique().collect();
    return result;
}

fn main() -> std::io::Result<()> {
    let source_path = Path::new(r#"C:\Users\Asus\Desktop\firefox_resolve\bookmarks_firefox_180829_2014_copy_copy.html"#);
    let source = get_from_html(source_path);

    let mut all_links = get_from_folder(Path::new(r#"C:\Users\Asus\Desktop\firefox_resolve\tabs"#));
    all_links.extend(
        &mut get_from_html(Path::new(r#"C:\Users\Asus\Desktop\bookmarks_firefox_191120_2052_noicons.html"#))
        .iter()
        .filter(|i| i.starts_with("http"))
        .map(String::from));
    
    all_links = all_links.iter().map(|i| {
        let j: Vec<&str> = i.split("://").collect();
        if j.len() > 1 {
            return j[1].to_string();
        } else {
            return j[0].to_string();
        }}).collect();
    // fs::write("C:\\Users\\Asus\\Desktop\\alltabs1.txt", all_links.join("\n")).expect("Unable write to file");

    //let a: PathBuf = source_path.parent().unwrap().join(source_path.file_stem().unwrap().to_str().unwrap().to_string().push_str("_uniq_links_rs.txt"));

    fs::write("C:\\Users\\Asus\\Desktop\\test.txt", del_existing(all_links, source).join("\n")).expect("Unable write to file");

    Ok(())
}