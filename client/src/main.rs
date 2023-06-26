use reqwest::{Client, Method, Response};

/// example code
///
/// Create a reqwest client
/// let client = Client::new();
/// Make a GET HTTP request to our backend's /example route
/// let res = client.get("http://localhost:8088/example").await?;
///
/// Get the response from backend's data
/// let body = res.text().await?;
/// Print out that response
/// println!("GET Response: {}", body);
///
/// Same as GET, but makes a POST request with appropriate header
/// let res = client
///     .post("http://localhost:8088/example")
///     .header("Content-Type", "application/json")
///     .body("Example Body")
///     .await?;
///
/// let body = res.text().await?;
/// println!("POST Response: {}", body);
///
/// You'll use these methods along with DELETE to accomplish your task
#[tokio::main]
async fn main() {
    // Create a reqwest client
    let client = Client::new();

   // Your code here!
}
