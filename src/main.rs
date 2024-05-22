mod login;
mod messages;
// mod counter;
pub mod counter;

use tokio;
use messages::get_messages;

#[tokio::main]
async fn main() {

    match login::login_basic().await {
        Ok(token) => {
            println!("Login successful, token: {}", token);
            match get_messages(&token).await {
                Ok(_) => println!("Messages retrieved successfully"),
                Err(e) => eprintln!("Error retrieving messages: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
