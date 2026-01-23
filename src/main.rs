use backend_and_database::run;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenvy::from_filename(".env").ok();
    } else {
        dotenvy::from_filename("prod.env").ok();
    }

    if let Err(e) = run().await {
        println!("error is {e}");
    }
}
