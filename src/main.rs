mod tool;

// async fn main() {
#[tokio::main]
async fn main(){
    println!("Hello, world!");

    // tool::run_main();
    tool::tool::run_main().await.unwrap();

    
}
