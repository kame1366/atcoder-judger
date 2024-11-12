fn get_samplecase(url_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let url= format!("https://atcoder.jp/contests/{url_id}/tasks/{url_id}");
    // let problem_id = vec!["_a"];

    //for id in problem_id {
    //    let problem_url = format!("{url}{id}");
    //    let body = reqwest::blocking::get(problem_url)?.text()?;
    //    let document = scraper::Html::parse_document(&body);
    //    let selector = scraper::Selector::parse("div.part > section > pre").unwrap();
    //    let element = document.select(&selector);
    //
    //    element.for_each(|e| println!("{}", e.text().next().unwrap()));
    //}

    let problem_url = format!("{url}_a");

    let body = reqwest::blocking::get(problem_url)?.text()?;
    let document = scraper::Html::parse_document(&body);

    let selector = scraper::Selector::parse("div.part").unwrap();
    let section = scraper::Selector::parse("section").unwrap();
    let pre = scraper::Selector::parse("pre").unwrap();

    let element: Vec<_> = document
        .select(&selector)
        .flat_map(|item| item.select(&section))
        .flat_map(|item| item.select(&pre))
        .map(|item| item.inner_html())
        .collect();

    println!("{:?}", element);

    Ok(())
}

pub fn download(url_id: String) {
    println!("Hello from download!");
    println!("I will download from {}!", url_id);

    let _ = get_samplecase(url_id);
}