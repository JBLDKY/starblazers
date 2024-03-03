use warp::Filter;

// The #[tokio::main] macro allows us to write an async main function by setting up the
// necessary asynchronous runtime provided by the Tokio library.
#[tokio::main]
async fn main() {
    // This defines a single GET route that matches the path "/hello".
    // When accessed, it responds with the text "Hello, world!".
    let routes = warp::path("hello").map(|| "Hello, world!");

    // This tells warp to serve the defined routes on localhost at port 3030.
    // The 'run' method will start a web server and await for incoming requests.
    // It binds to the specified IP and port and starts handling requests asynchronously.
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await; // The .await is required because 'run' is an async operation.
}
