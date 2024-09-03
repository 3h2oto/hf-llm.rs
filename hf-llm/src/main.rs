use clap::{Command, Arg};
use hf_hub::Cache;
use reqwest::Client;
use serde_json::json;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let matches = Command::new("hf-llm.rs")
        .version("0.1.0")
        .author("VB")
        .about("A CLI to access LLMs hosted on Hugging Face")
        .arg(Arg::new("model-name")
            .short('m')
            .long("model-name")
            .value_name("MODEL")
            .help("Specify the Hugging Face Hub ID of the model to use.")
            .required(true)
            )
        .arg(Arg::new("prompt")
            .short('p')
            .long("prompt")
            .value_name("PROMPT")
            .help("Specify the prompt to use.")
            .required(true)
            )
        .get_matches();

    let model_name = matches.get_one::<String>("model-name").unwrap();
    let prompt = matches.get_one::<String>("prompt").unwrap();

    let cache = Cache::default();

    if let Some(token) = cache.token() {
        let url = format!("https://api-inference.huggingface.co/models/{}/v1/chat/completions", model_name);
        
        let client = Client::new();
        let res = client
           .post(url)
           .header("Authorization", format!("Bearer {}", token))
           .header("Content-Type", "application/json")
           .json(&json!({
                "model": model_name,
                "messages": [{"role": "user", "content": prompt}],
                "max_tokens": 500,
                "stream": false
            }))
           .send()
           .await?;
        
        let response = res.json::<serde_json::Value>().await?;
        println!("{:?}", response);
        
        Ok(())
    } else {
        println!("Token not found, please run `huggingface-cli login`");
        std::process::exit(1);
    }
}