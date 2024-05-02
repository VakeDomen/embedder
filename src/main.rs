use embedding::generate_prompt_embedding;

mod qdrant;
mod config;
mod embedding;
mod loader;

#[tokio::main]
async fn main() {
    match generate_prompt_embedding("Hello world!").await {
        Ok(em) => println!("{:#?}", em),
        Err(e) => panic!("{:#?}", e),
    };
}
