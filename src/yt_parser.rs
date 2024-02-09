use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug)]
enum UrlType {
    Playlist,
    List,
    Video,
}
pub struct CustomResult {
    urls: Vec<(String, UrlType, bool)>,
}

impl CustomResult {
    pub fn display(&self) {
        for (a, b, c) in &self.urls {
            println!("-------------------------------------------------");
            println!("URL: {:?} \nTYPE:{:?} \nDownloaded:{:?}", a, b, c);
            println!("-------------------------------------------------");
        }
    }
}
/*
* returns:
*   ownership of array of this type
*       url string, url type enum, bool for download success/nil
*   and error/nil
*/
pub fn custom_parser(file_name: &String) -> CustomResult {
    let mut res = CustomResult { urls: Vec::new() };
    let opens = BufReader::new(File::open(file_name).expect("Error in opening file"));
    for line_with_err in opens.lines() {
        let line = line_with_err.expect("err while asserting string");
        if line.contains("youtube") && line.contains("/playlist") {
            res.urls.push((line, UrlType::Playlist, false))
        } else if line.contains("youtube") && line.contains("&list") {
            res.urls.push((line, UrlType::List, false))
        } else if line.contains("youtube") && line.contains("/watch") {
            res.urls.push((line, UrlType::Video, false))
        }
    }
    res
}
