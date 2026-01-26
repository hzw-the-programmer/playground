use anyhow::{Error as E, Result};
use tokenizers::tokenizer::Tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从单个json文件加载分词器
    let tokenizer =
        Tokenizer::from_file("D:\\modules\\starcoderbase-1b\\tokenizer.json").map_err(E::msg)?;

    // 使用分词器
    let encoding = tokenizer.encode("Hello, world!", false).map_err(E::msg)?;
    println!("Tokens: {:?}", encoding.get_tokens());
    println!("IDs: {:?}", encoding.get_ids());

    Ok(())
}
