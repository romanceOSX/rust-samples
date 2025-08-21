use trpl::Html;

// What happends when rust stumbles across an async function?
//  It rewrites the function to return a single async block as the following:
// This is the equivalent function signature:
//  fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//      async move {
//          ...
//      }
//  }
//  In turn, an async blocks makes rust to compile into an unique anonymus data type
//  that implements the Future trait
//
//  So an async function gets turned into a non-async function that returns a type
//  that implements a Future trait associated with the return type of your code...
//
//  Basically wraps the function within an async move block and returns a future type
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

use std::time::Duration;

fn test_spawning_tasks() {
    // async block compiles to anonymus functions, we can rewrite the following
    trpl::run(async {
        // not necessary
        let task_handle1 = trpl::spawn_task(async {
            for i in 1..10 {
                println!("Hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("Hi number {i} from the seconds task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
        task_handle1.await.unwrap();
    });
    
    // like this
    trpl::run(async {
        let f1 = async {
            for i in 1..5 {
                println!("Hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let f2 = async {
            for i in 1..10 {
                println!("Hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // in contrast to threads, we will see that this makes them run in a fixed
        // predictable fashion, this is due to the asynchronous runtime handling the
        // execution, in contrast with threads where the OS is in charge of the sceduling
        // aka. the application knows more about concurrency handling
        trpl::join(f1, f2).await;
    });
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

fn test_async_message_passing() {
    let run = async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    };

    trpl::run(run);
}

fn main() {
    test_async_message_passing();
    test_spawning_tasks();
    run_async();
    println!("Running on main")
}

