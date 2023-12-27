use std::io;
use reqwest::blocking::get;
use scraper::{Html, Selector};

#[allow(dead_code)]

/* TODO:
    - Multiple paragraph sections per header
        - Does not align with list
        - Cannot reach paragraphs at the end
*/
fn main() {
    // Specify the URL of the website you want to scrape
    let url = "https://en.wikipedia.org/wiki/Lyngen_Alps";

    // Fetch the HTML content of the website
    let body = match get(url) {
        Ok(response) => response.text().unwrap(),
        Err(err) => panic!("Error fetching the website: {}", err),
    };

    // Parse the HTML content
    let document = Html::parse_document(&body);

    // Define a CSS selector to locate the titles of articles
    let title_selector = Selector::parse("h2").unwrap();
    let body_selector = Selector::parse("p").unwrap();

    // Extract and print the titles
    let mut section_count = 1;
    println!("0: Quit");
    for title in document.select(&title_selector) {
        println!("{section_count}: {}", title.text().collect::<String>());
        section_count += 1;
    }

    // Print body given list of titles
    loop {
        println!("Which paragraph would you like to see?");
        let mut body_num = String::new();
        io::stdin()
            .read_line(&mut body_num)
            .expect("Failed to read guess!");
        let body_num: i32 = body_num.trim().parse().expect("Please type a number!");

        if body_num == 0 {
            break;
        }
        if body_num >= 1 && body_num <= section_count {
            let mut body_iterator = 1;
            for body in document.select(&body_selector) {
                if body_iterator == body_num {
                    println!("{}", body.text().collect::<String>());
                }
                body_iterator += 1;
            }
        }
    }

}
