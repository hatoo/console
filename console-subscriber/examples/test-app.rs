use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    console_subscriber::init();

    let futures = (0..10)
        .map(|_| {
            tokio::spawn(async {
                loop {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            })
        })
        .collect::<Vec<_>>();

    for f in futures {
        f.await?;
    }

    Ok(())
}
