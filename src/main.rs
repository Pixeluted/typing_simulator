use std::io;
use std::io::{BufRead};
use tokio::time::{sleep, Duration};

async fn validate_discord_token(token: String) -> Result<bool, ()> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://discord.com/api/v9/users/@me")
        .header("Authorization", token.trim())
        .send()
        .await
        .expect("Failed to carry out the request!");


    Ok(response.status().is_success())
}

async fn validate_channel_id(channel_id: String, token: String) -> Result<bool, ()> {
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("https://discord.com/api/v9/channels/{}", channel_id))
        .header("Authorization", token.trim())
        .send()
        .await
        .expect("Failed to carry out the request!");

    Ok(response.status().is_success())
}

async fn setup() -> Result<(String,String), ()> {
    let mut discord_token = String::new();
    let mut discord_token_clone = String::new();
    let mut discord_token_third_clone = String::new();
    let mut channel_id = String::new();
    let mut channel_id_clone = String::new();

    let mut token_done = false;
    let mut channel_id_done = false;

    println!("In order for this application to work, we need your discord token so we can simulate typing status.");
    println!("Please enter token here: ");

    match io::stdin().read_line(&mut discord_token) {
        Ok(_) => {
            discord_token_clone = discord_token.clone();
            discord_token_third_clone = discord_token.clone();
            let is_token_valid = validate_discord_token(discord_token).await?;

            if is_token_valid {
                println!("Sucessfully set the token!");
                token_done = true;
            } else {
                println!("This token is not valid!");
            }
        }
        Err(err) => {
            println!("Failed to read the line! Error: {}", err.to_string());
        }
    }
    
    if token_done {
        println!("Please enter the channel id you want to type in: ");
        match io::stdin().read_line(&mut channel_id) {
            Ok(_) => {
                channel_id_clone = channel_id.clone();
                let is_channel_id_valid = validate_channel_id(channel_id, discord_token_clone).await?;

                if is_channel_id_valid {
                    println!("Sucessfully set the channel id!");
                    channel_id_done = true;
                } else {
                    println!("This channel id is not valid!");
                }
            }
            Err(err) => {
                println!("Failed to read the line! Error: {}", err.to_string());
            }
        }

        if channel_id_done {
            return Ok((discord_token_third_clone.to_string(), channel_id_clone.to_string()));
        } else {
            return Err(());
        }
    } else {
        return Err(());
    }
}

async fn start_loop(discord_token: String, channel_id: String) {
    let client = reqwest::Client::new();

    loop {
        let response = client
            .post(format!("https://discord.com/api/v9/channels/{}/typing", channel_id))
            .header("Authorization", discord_token.trim())
            .send()
            .await;

        println!("Request sent!");
        
        if response.is_err() {
            println!("Failed to sent request! Err: {}", response.err().unwrap().to_string())
        }

        sleep(Duration::from_secs(10)).await;
    }
}


#[tokio::main]
async fn main() {
    let settuped = setup().await;

    if !settuped.is_err() {
        let (discord_token, channel_id) = settuped.unwrap();

        println!("We have started!");
        start_loop(discord_token, channel_id).await;
    } else {
        println!("Please try again!")
    }

    let stdin = io::stdin();
    let _ = stdin.lock().lines().next();
}
