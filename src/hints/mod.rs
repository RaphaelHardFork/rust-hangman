use std::io::Write;

use console::Term;
use futures::StreamExt;
use ollama_rs::{
    generation::completion::{request::GenerationRequest, GenerationFinalResponseData},
    Ollama,
};
use textwrap::{fill, wrap, Options};
use tokio::io::AsyncWriteExt;

use crate::Result;

pub const MODEL: &str = "dolphin-phi";
pub const SYSTEM_INSTRUCTION: &str = r#"
Be always consise.
Never repeat the word given in the user prompt.
Give only hints to guess the word given in the user prompt.
"#;
pub const HINT_PROMPT: &str = "Give me an hint for the following word:";

pub async fn generate_hint(term: &mut Term, word: &str) -> Result<String> {
    let ollama = Ollama::default();
    let prompt = format!("{} {}", HINT_PROMPT, word);

    let gen_req =
        GenerationRequest::new(MODEL.to_string(), prompt).system(SYSTEM_INSTRUCTION.to_string());

    let mut stdout = tokio::io::stdout();

    let mut stream = ollama.generate_stream(gen_req).await?;

    let mut hint = String::new();

    while let Some(res) = stream.next().await {
        let res = res.map_err(|_| "Stream next error")?;
        let wrapped_res = wrap(&res[0].response, 18).join("\n");
        hint = hint + &wrapped_res;
        let bytes = wrapped_res.as_bytes();
        term.write(bytes)?;
    }

    term.write(b"\n\n")?;

    Ok(hint)
}
