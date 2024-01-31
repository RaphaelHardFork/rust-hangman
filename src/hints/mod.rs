use crate::Result;
use console::Term;
use futures::StreamExt;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;
use std::io::Write;
use textwrap::wrap;

pub const MODEL: &str = "dolphin-phi";
pub const SYSTEM_INSTRUCTION: &str = r#"
Consider the word in the prompt as a valid english word. 
Give one or several hints to guess this word.
ALWAYS replace letter of this word by '*' if you need to use it.
Be consise. Skip describing you you are doing.
"#;
pub const HINT_PROMPT: &str = "";

pub async fn generate_hint(term: &mut Term, word: &str) -> Result<String> {
    let ollama = Ollama::default();
    let prompt = format!("{} {}", HINT_PROMPT, word);

    let gen_req =
        GenerationRequest::new(MODEL.to_string(), prompt).system(SYSTEM_INSTRUCTION.to_string());

    let mut stream = ollama.generate_stream(gen_req).await?;

    let mut hint = String::new();

    while let Some(res) = stream.next().await {
        let res = res.map_err(|_| "Stream next error")?;
        let wrapped_res = wrap(&res[0].response, 18).join("\n");
        hint = hint + &wrapped_res;
        let bytes = wrapped_res.as_bytes();
        term.write_all(bytes)?;
    }

    term.write_all(b"\n\n")?;

    Ok(hint)
}
