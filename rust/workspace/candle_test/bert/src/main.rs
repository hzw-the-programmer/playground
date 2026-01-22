use candle_core::{Device, Result, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::bert::{BertModel, Config, DTYPE, HiddenAct};
use tokenizers::{PaddingParams, Tokenizer};

fn main() {
    let device = Device::Cpu;

    let vb = VarBuilder::from_pth(
        "D:\\modules\\bge-large-zh-v1.5\\pytorch_model.bin",
        DTYPE,
        &device,
    )
    .unwrap();

    let config = std::fs::read_to_string("D:\\modules\\bge-large-zh-v1.5\\config.json").unwrap();
    let mut config: Config = serde_json::from_str(&config).unwrap();

    let model = BertModel::load(vb, &config).unwrap();

    let mut tokenizer = Tokenizer::from_file("D:\\modules\\bge-large-zh-v1.5\\tokenizer.json").unwrap();
    let tokenizer = tokenizer.with_padding(None).with_truncation(None).unwrap();
    let tokens = tokenizer
        .encode("Here is a test sentence", true)
        .unwrap()
        .get_ids()
        .to_vec();
    let token_ids = Tensor::new(&tokens[..], &device)
        .unwrap()
        .unsqueeze(0)
        .unwrap();
    let token_type_ids = token_ids.zeros_like().unwrap();
    let ys = model.forward(&token_ids, &token_type_ids, None).unwrap();
    
    println!("{ys}");
    println!("hello");
}
