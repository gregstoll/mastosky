use megalodon::{entities::Status, response::Response};
use thiserror::Error;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let (username, instance) = parse_full_username("@wojespn@sportsbots.xyz").unwrap();
    let posts = get_mastodon_posts(username, instance).await.unwrap();
    //let posts = get_mastodon_posts("@wojespn@sportsbots.xyz", "techhub.social").await.unwrap();
    println!("{}", format!("status: {}", posts.status));
    println!("{}", format!("status_text: {}", posts.status_text));
}

#[derive(Error, Debug)]
pub enum ParseUsernameError {
    #[error("username is empty")]
    Empty,
    #[error("username is not ASCII")]
    NotAscii,
    #[error("username does not start with @")]
    DoesNotStartWithAt,
    #[error("username does not contain a second @")]
    DoesNotContainAt,
}

fn parse_full_username(full_username: &str) -> Result<(&str, &str), ParseUsernameError> {
    if full_username.is_empty() {
        return Err(ParseUsernameError::Empty)
    }
    if !full_username.is_ascii() {
        return Err(ParseUsernameError::NotAscii)
    }
    if !full_username.starts_with('@') {
        return Err(ParseUsernameError::DoesNotStartWithAt)
    }
    let at_index = full_username[1..].find('@').ok_or(ParseUsernameError::DoesNotContainAt)?;
    let username = &full_username[1..at_index];
    let instance = &full_username[at_index+1..];
    return Ok((username, instance));
}

async fn get_mastodon_posts(username: &str, instance: &str) -> Result<Response<Status>, megalodon::error::Error> {
    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        format!("https://{}", instance),
        None,
        Some(String::from("github.com/gregstoll/megalosky"))
      );
    
    //let x = client.get_status(String::from("@wojespn@sportsbots.xyz/112493182813741400")).await?;
    let x = client.get_status(String::from("wojespn/112493182813741400")).await?;
    Ok(x)
}