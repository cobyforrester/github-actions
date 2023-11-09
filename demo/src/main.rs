use warp::Filter;

#[tokio::main]
async fn main() {
    // Define a simple route that handles GET requests on the root ("/") URL
    let hello_route = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    // Start the warp server on port 3030
    warp::serve(hello_route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}
