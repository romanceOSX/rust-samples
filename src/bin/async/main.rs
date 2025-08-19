use trpl::Html;

// This is the equivalent function signature:
//  fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//      async move {
//          ...
//      }
//  }
//  Basically wraps the function within an async move block and returns a future type
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

fn run_async() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title of the {url} was {title}"),
            None => println!("{url} had no title..."),
        }
    })
}

fn main() {
    run_async();
    println!("Running on main")
}

