use backend_and_database::run;
use dotenvy::dotenv;


#[tokio::main]
async fn main() {
    // if cfg!(debug_assertions) {
        dotenv().ok();
        // println!("Loaded local .env file");
    // }else{

    // }
    
   if let Err(e) = run().await{
    println!("error is {e}");
   }
}








