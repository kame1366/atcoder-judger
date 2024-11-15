use std::error::Error;

use crate::export;

fn get_samplecase(url: String) -> Result<Vec<String>, Box<dyn Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let document = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("span.lang-ja > div.part > section > pre").unwrap();
    let element: Vec<_> = document
        .select(&selector)
        .map(|item| item.inner_html())
        .collect();

    Ok(element)
}

fn save(contest_id: String, problem_id: String) {
    let url = format!("https://atcoder.jp/contests/{contest_id}/tasks/{contest_id}_{problem_id}");

    let samplecase = match get_samplecase(url) {
        Ok(samlecase) => samlecase,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    if samplecase.is_empty() {
        return;
    }

    export::export(contest_id, problem_id, samplecase);

}

fn get_from_id(contest_id: String) {
    let contest_id = contest_id.to_lowercase();
    const PROBLEM_ID: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

    for id in PROBLEM_ID {
        save(contest_id.clone(), String::from(id));
    }
}

pub fn download(contest_id: String) {
    println!("Downloading from {}...", contest_id);
    get_from_id(contest_id);
}
