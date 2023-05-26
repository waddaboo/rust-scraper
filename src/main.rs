use scraper::{Html, Selector};

fn get_hacker_news_data() -> Result<String, Box<dyn std::error::Error>> {
    let text = reqwest::blocking::get("https://news.ycombinator.com/")?.text()?;

    Ok(text)
}

fn main() {
    let text = get_hacker_news_data().unwrap();

    let document = Html::parse_document(&text);

    let stories = Selector::parse("td:nth-child(3) > span > a").unwrap();

    for story in document.select(&stories) {
        let story_link = story.value().attr("href").unwrap();
        let story_text = story.text().collect::<Vec<_>>();

        if story_text[0] == "login" {
            continue;
        }

        println!("{} | {}", story_text[0], story_link)
    }
}
