fn get_samplecase(url_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://atcoder.jp/contests/{url_id}/tasks/{url_id}");
    let problem_id = vec!["a", "b", "c", "d", "e", "f", "g"];

    for id in problem_id {
        let problem_url = format!("{url}_{id}");
        let body = reqwest::blocking::get(problem_url)?.text()?;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("span.lang-ja > div.part > section > pre").unwrap();
        let element: Vec<_> = document.select(&selector).map(|item| item.inner_html()).collect();
        element.iter().for_each(|e| println!("{}", e));
    }

    Ok(())
}

pub fn download(url_id: String) {
    println!("Hello from download!");
    println!("I will download from {}!", url_id);

    let _ = get_samplecase(url_id);
}
