use warp::Filter;
use serde::Deserialize;
mod db;
use db::Book;

const DB: &'static str = "database.db";

// Statically include the web content
const HTML: &'static str = include_str!("../assets/app.html");
const JS: &'static str = include_str!("../assets/app.js");
const KJS: &'static str = include_str!("../assets/katex.min.js");
const CSS: &'static str = include_str!("../assets/app.css");
const KCSS: &'static str = include_str!("../assets/katex.min.css");

#[derive(Deserialize)]
struct SearchQuery {
    q: String,
}

#[derive(Deserialize)]
struct AddQuery {
    desc: String,
    eqn: String,
}

#[tokio::main]
async fn main() {
    // Static routes (for files)
    let html = warp::get()
        .map(|| warp::reply::html(HTML));
    let js = warp::path!("app.js")
        .map(|| JS)
        .map(|reply| warp::reply::with_header(reply, "Content-Type", "application/javascript"));
    let kjs = warp::path!("katex.min.js")
        .map(|| KJS)
        .map(|reply| warp::reply::with_header(reply, "Content-Type", "application/javascript"));
    let css = warp::path!("app.css")
        .map(|| CSS)
        .map(|reply| warp::reply::with_header(reply, "Content-Type", "text/css"));
    let kcss = warp::path!("katex.min.css")
        .map(|| KCSS)
        .map(|reply| warp::reply::with_header(reply, "Content-Type", "text/css"));

    // add endpoint
    let add_equation = warp::path!("add")
        .and(warp::query::<AddQuery>())
        .map(|query: AddQuery| {
            let book = Book::open(DB).expect("Error opening database.");
            book.add_equation(&query.desc, &query.eqn).expect("Error adding equation.");
            "success"
        });

    // search endpoint
    let search = warp::path!("search")
        .and(warp::query::<SearchQuery>())
        .map(|query: SearchQuery| {
            let book = Book::open(DB).expect("Error opening database.");
            let results = book.search(&query.q).unwrap();
            warp::reply::html(results.iter().map(|eqn| format!(r#"
                <li><div class="description">{}</div><div class="equation">{}</div></li>
            "#, eqn.html_description(), eqn.html_equation(true))).collect::<String>())
        });

    // combine everything
    let get = warp::get().and(search.or(js).or(kjs).or(css).or(kcss).or(html));
    let post = warp::post().and(add_equation);

    warp::serve(get.or(post))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
