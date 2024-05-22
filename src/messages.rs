

use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::Write;
use crate::counter::counter::message_counter;

pub async fn get_messages(auth_token: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let res = client
        .get("https://api.staging.orionmessenger.io/api/messages?category=0&chat_id=7a793ef1-5a3d-45c5-934c-04e25270046e&sent_before=2024-04-20T02%3A43%3A07.337%2B00%3A00&category1=3&take=20")
        .header(AUTHORIZATION, format!("Bearer {}", auth_token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .text()
        .await?;


    let mut file = File::create("get_message_res.txt")?;
    file.write_all(res.as_bytes())?;

    let contents = fs::read_to_string("get_message_res.txt")?;
    
    println!("input user handle:");

    let mut user_handle = String::new();
     std::io::stdin().read_line(&mut user_handle).unwrap();

  
    message_counter(&user_handle, &contents);
    Ok(())
}
