use std::fs;

pub fn demo_html2md() {
    let url = "https://www.rust-lang.org/"; 
    let output = "rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Fetching body: {}", body);

    println!("Converting html to markdown..."); 
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes());
    println!("Converted markdown has been saved in {}.", output);
}