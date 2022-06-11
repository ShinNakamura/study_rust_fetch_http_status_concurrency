use rayon::prelude::*;
use std::io::{self, BufRead};
use std::{thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut input = io::BufReader::new(input.lock());
    let mut url_list: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        let bytes = input.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        url_list.push(line.trim().to_string());
    }
    println!("url,status");
    let duration_millis = time::Duration::from_millis(100);
    url_list.par_iter()
        .for_each(|url| {
            let resp = reqwest::blocking::get(url).unwrap();
            thread::sleep(duration_millis); // sleepを挟んでリクエストを緩くする
            println!("{},{:?}", url, resp.status());
        });
    Ok(())
}
