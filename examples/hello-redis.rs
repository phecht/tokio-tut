use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open connection
    let mut client = client::connect("127.0.0.1:6379").await?;

        // setting a key hello with world
    client.set("Hello","world".into()).await?;

    let result = client.get("Hello").await?;

    println!("got value from the server; result={:?}", result);
    Ok(())

}