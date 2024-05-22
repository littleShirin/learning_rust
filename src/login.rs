use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::fs::File;
use std::io::Write;
use std::error::Error;

#[derive(Deserialize)]
pub struct ResponseBody {
    pub token: String,
}

pub async fn login_basic() -> Result<String, Box<dyn Error>> {
    let payload = json!({
        "username": "shirinlittle",
        "password": "$argon2id$v=19$m=19456,t=2,p=1$c2hpcmlubGl0dGxl$JIrpiDSXQ6GbvL2Fnlh7gCxGh/FZmJYXF9jkBCDaVrE",
    });

    // Create a new reqwest client
    let client = Client::new();

    // POST request 
    let res = client.post("https://api.staging.orionmessenger.io/register/login_basic")
        .json(&payload)
        .send()
        .await?;

    // Check if the request was successful
    if res.status().is_success() {
        println!("Request was successful");
    } else {
        println!("Request failed with status: {}", res.status());
    }

    let body = res.text().await?;

    // Parse the response body as JSON
    let response_body: ResponseBody = serde_json::from_str(&body)?;

    // Extract the token
    let token = response_body.token;

    // Write the response body to a text file
    let mut file = File::create("response.txt")?;
    file.write_all(body.as_bytes())?;

    Ok(token)
}

// return value Result represents either success(Ok) or failure(Err).
// Ok(()): Ok is a variant of Result that indicates success, and () is an empty tuple, which is used when there is no useful value to return.
// .await? -> ? operator is used to propagate errors up the call stack. If an error occurs, the function will return early with the error value. Unwrapping the value of Result 
// Box<dyn std::error::Error> in case of an error returned from the main function, the error is boxed and returned as a trait object. This allows the function to return different types of errors.
// Box is a smart pointer that points to heap-allocated data. It is used to store values of unknown size at compile time. The dyn keyword is used to create a trait object, which allows the function to return different types of errors.