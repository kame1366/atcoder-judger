use std::error::Error;

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

fn get_from_id(url_id: String) -> Result<(), Box<dyn Error>> {
    let url = format!("https://atcoder.jp/contests/{url_id}/tasks/{url_id}");
    let problem_id = vec!["a", "b", "c", "d", "e", "f", "g"];

    for id in problem_id {
        let element = get_samplecase(format!("{url}_{id}"))?;
        element.iter().for_each(|e| println!("{}", e));
    }

    Ok(())
}

pub fn download(url_id: String) {
    println!("Hello from download!");
    println!("I will download from {}!", url_id);

    let _ = get_from_id(url_id);
}
