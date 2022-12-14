use std::fs;

pub fn demo_html2md() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    let output = &args[2];

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted markdown has been saved in {}.", output);
}
