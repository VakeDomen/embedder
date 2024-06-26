use anyhow::{Error, Result};
use candle_core::Tensor;

use crate::loader::load_bert_model;


/// Generates a normalized L2 embedding for a given text prompt using a pre-loaded BERT model.
///
/// This function loads a BERT model and tokenizer, encodes the input prompt into token IDs, and uses the model
/// to generate word embeddings. The embeddings are then pooled (averaged) and normalized to produce a single
/// vector representation of the prompt, suitable for further processing or similarity comparisons.
///
/// # Parameters
/// - `prompt`: The text prompt to be embedded.
///
/// # Returns
/// Returns a `Result` containing the normalized tensor representing the L2 embedding of the prompt.
///
/// # Errors
/// - Returns an error if any step in the embedding generation process fails, including model loading, tokenization,
///   tensor operations, or WebSocket communication failures.
pub async fn generate_prompt_embedding(
    prompt: &str,
    ) -> Result<Tensor> {
    let (model, tokenizer, device) = load_bert_model(Some(0))?;
    let tokens = tokenizer
            .encode(prompt, true)
            .map_err(Error::msg)?
            .get_ids()
            .to_vec();
    let token_ids = Tensor::new(&tokens[..], &device)?.unsqueeze(0)?;
    let token_type_ids = token_ids.zeros_like()?;
    let embeddings = model.forward(&token_ids, &token_type_ids)?;
    // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
    let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
    let embeddings = normalize_l2(&embeddings)?;
    
    Ok(embeddings)
} 

/// Normalizes a tensor using L2 norm.
///
/// This function takes a tensor and normalizes its values across a specified dimension using the L2 norm.
/// This is typically used to normalize embeddings so that they can be more effectively compared or processed
/// downstream.
///
/// # Parameters
/// - `v`: The tensor to be normalized.
///
/// # Returns
/// Returns a `Result` containing the normalized tensor.
///
/// # Errors
/// - Returns an error if tensor operations required for normalization fail.
pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}