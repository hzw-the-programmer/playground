use openai_api_rust::chat::*;
use openai_api_rust::completions::*;
use openai_api_rust::*;

fn main() {
    // Load API key from environment OPENAI_API_KEY.
    // You can also hadcode through `Auth::new(<your_api_key>)`, but it is not recommended.
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(7),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: "Hello!".to_string(),
        }],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();
    assert!(message.content.contains("Hello"));
}
