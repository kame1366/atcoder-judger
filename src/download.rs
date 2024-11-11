fn get_samplecase(url_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let url= format!("https://atcoder.jp/contests/{url_id}/tasks/{url_id}");
    let problem_id = vec!["_a", "_b", "_c", "_d", "_e", "_f", "_g"];
    // let problem_url: Vec<String> = vec![];

    for id in problem_id {
        let problem_url = format!("{url}{id}");
        let body = reqwest::blocking::get(problem_url)?.text()?;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("div.part > section > pre").unwrap();
        let elements = document.select(&selector);  

        elements.for_each(|e| println!("{}", e.text().next().unwrap()));
    }
    Ok(())
}

pub fn download(url_id: String) {
    println!("Hello from download!");
    println!("I will download from {}!", url_id);

    let _ = get_samplecase(url_id);
}