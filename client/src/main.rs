use serde_json::json;
use reqwest::Client;
///use reqwest::{Client, Method, Response};

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
async fn main() -> anyhow::Result<()> {

    // Your code here!
    // bug_1().await?;

    // bug_2().await?;

    // bug_3().await?;

    bug_4().await?;

    // You'll use these methods along with DELETE to accomplish your task
    Ok(())
}

///
/// GET/questions creates a post.
/// This is best seen by calling GET/questions twice.
/// Each time, a new post is created.
async fn bug_1() -> anyhow::Result<()> {
    // Print out that response
    println!("\n
        \nBug 1:\
        \nWe expect the list of questions to be empty.
    ");

    get_all_questions("1").await?;

    get_all_questions("2").await?;

    Ok(())
}


///
/// get_all creates a duplicate question with the same non-unique id.
async fn bug_2() -> anyhow::Result<()> {
    // Print out that response
    println!("
        \nBug 2:\
        \nWe expect to never see two identical ids.
    ");

    get_all_questions("1").await?;

    delete_question("1").await?;

    get_all_questions("2").await?;

    println!("\n");
    Ok(())
}


///
/// two POSTs creates a duplicate question with the same non-unique id.
async fn bug_3() -> anyhow::Result<()> {
    // Print out that response
    println!("
        \nBug 3:\
        \nWe expect to never see two identical ids.
    ");

    post_question("0").await?;

    post_question("1").await?;

    delete_question("1").await?;

    get_all_questions("1").await?;

    println!("\n");
    Ok(())
}

///
/// Delete with the argument of the id,
/// should delete only the post with id
/// but this bug deletes all posts
/// except for the id argument.
///
async fn bug_4() -> anyhow::Result<()> {
    // Print out that response
    println!("
        \nBug 3:\
        \nWe expect to never see two identical ids.
    ");

    post_question("0").await?;

    post_question("1").await?;

    post_question("2").await?;

    post_question("3").await?;

    post_question("4").await?;

    delete_question("1").await?;

    get_all_questions("1").await?;

    println!("\n");
    Ok(())
}

/// ------------------------------------------------------------
/// --------------------API HELPER FUNCTIONS--------------------
/// ------------------------------------------------------------
/// Below are helper functions to make bug writing easier.

async fn get_all_questions(num: &str) -> anyhow::Result<()>  {
    // Create a reqwest client
    let client = Client::new();

    // Make a GET HTTP request to our backend's /example route
    let res = client.get("http://localhost:8088/questions")
        .send()
        .await?;

    // Get the response from backend's data
    let body = res.text().await?;

    // Print out that response
    println!("GET All Questions #{}: \n{}", num, body);

    Ok(())
}

///
/// Used the format() string concatention from this rust article:
/// https://docs.rs/serde_json/latest/serde_json/
async fn post_question(num_str: &str) -> anyhow::Result<()>  {
    // Create a reqwest client
    let client = Client::new();

    // The type of `john` is `serde_json::Value`
    let body_json = json!({
        "id": "41",
        "title": format!("Bug Question {}", num_str),
        "content": format!("Bug Content {}", num_str),
        "tags": [
            format!("Bug Tag {}", num_str)
        ]
    });

    // Same as GET, but makes a POST request with appropriate header
    let res = client
        .post("http://localhost:8088/question?question_id=2")
        .header("Content-Type", "application/json")
        .body(body_json.to_string())
        .send()
        .await?;

    let body = res.text().await?;
    println!("POST Question: {}", body);

    Ok(())
}

async fn delete_question(num_str: &str) -> anyhow::Result<()>  {

    let client = Client::new();

    // Make a GET HTTP request to our backend's /example route
    let res = client
        .delete(format!("http://localhost:8088/question?question_id={}", num_str))
        .send()
        .await?;

    // Get the response from backend's data
    let body = res.text().await?;

    // Print out that response
    println!("DELETE question id={}: \n{}", num_str, body);

    Ok(())
}

