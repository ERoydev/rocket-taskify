use rocket::{fairing::{Fairing, Info, Kind}, http::Status, Orbit, Rocket};
use tokio::time::{interval, Duration};
use reqwest::Client as ReqClient;

use crate::ErrorResponder;


pub struct CleanupTask;

#[rocket::async_trait]
impl Fairing for CleanupTask {
    fn info(&self) -> Info {
        Info {
            name: "Revoked Tokens Cleanup Task",
            kind: Kind::Liftoff,
        }
    }

    async fn on_liftoff(&self, rocket: &Rocket<Orbit>) {
        println!("🚀 Rocket has lifted off!");

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60)); // Runs every 1 hour
            loop {
                interval.tick().await;
                match  cleanup_expired_tokens().await {
                    Ok(_) => println!("✅ Expired revoked tokens cleaned up successfully!"),
                    Err(err) => eprintln!("❌ Error cleaning revoked tokens: {:?}", err),
                }
            }
        });
    }
}

async fn cleanup_expired_tokens() -> Result<(), ErrorResponder> {

    let client = ReqClient::new();

    let api_url = "http://localhost:8000/auth/clean_expired_tokens"; // ================================================= API_URL FOR UPDATING PRIORITIES =================

    let response = client.post(api_url)
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                println!("Successfully updated revoked_tokens table:");
                Ok(())
            } else {
                println!("Failed to delete expired tokens from RevokedTokens table! Status: {}", res.status());
                Err(ErrorResponder::new("Failed to delete expired tokens from RevokedTokens table", Status::InternalServerError))
            }
        }
        Err(_) => {
            Err(ErrorResponder::new("Error making request", Status::InternalServerError))
        }
    }

}
