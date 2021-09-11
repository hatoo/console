use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    console_subscriber::init();

    let futures = (0..10/* A smaller number like 5 was Ok */)
        .map(|_| {
            tokio::spawn(async {
                loop {
                    tokio::time::sleep(Duration::from_millis(
                        10, /* A larger number like 1000 was Ok */
                    ))
                    .await;
                }
            })
        })
        .collect::<Vec<_>>();

    for f in futures {
        f.await?;
    }

    Ok(())
}
