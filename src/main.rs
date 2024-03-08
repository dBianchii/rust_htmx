use axum::{response::Html, routing::get, Router};
use std::sync::Mutex;
use lazy_static::lazy_static;

// Define a global shared state for storing tasks
lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}


#[tokio::main]
async fn main() {
    

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    
    Html("<h1>Hello, World!</h1>")
}
