use anyhow::Result;
use serde::{Deserialize, Serialize};
use wstd::{
    http::{Client, IntoBody, Request},
    io::AsyncRead,
};

// Request structure for stable diffusion
#[derive(Serialize, Deserialize)]
struct StableDiffusionRequest {
    prompt: String,
    negative_prompt: String,
    seed: i64,            // Fixed seed for determinism
    steps: u32,           // Number of inference steps
    cfg_scale: f32,       // Guidance scale
    width: u32,           // Image width
    height: u32,          // Image height
    sampler_name: String, // Specific sampler to use
    model: String,        // Specific model checkpoint
}

// Response structure with flexible parameter handling
#[derive(Deserialize, Debug)]
struct StableDiffusionResponse {
    images: Vec<String>, // Base64 encoded images
    #[serde(default)] // Make parameters optional
    parameters: Option<serde_json::Value>, // Use generic Value to handle any response structure
}

/// Generate a deterministic image using Stable Diffusion API
pub async fn generate_deterministic_image(prompt: &str) -> Result<String, String> {
    // Get API URL from environment variable
    let api_url = std::env::var("WAVS_ENV_SD_API_URL")
        .unwrap_or_else(|_| "http://localhost:7860/sdapi/v1/txt2img".to_string());

    // Get API key from environment variable
    let api_key = std::env::var("WAVS_ENV_SD_API_KEY").unwrap_or_default();

    // Fixed parameters for deterministic generation
    let request_data = StableDiffusionRequest {
        prompt: prompt.to_string(),
        negative_prompt: "blurry, bad quality, distorted".to_string(),
        seed: 42, // Always use the same seed
        steps: 30,
        cfg_scale: 7.0,
        width: 512,
        height: 512,
        sampler_name: "DPM++ 2M Karras".to_string(),
        model: "v1-5-pruned-emaonly".to_string(), // Match the model specified by the user
    };

    // Serialize to JSON
    let json_data = serde_json::to_string(&request_data)
        .map_err(|e| format!("JSON serialization error: {}", e))?;

    // For debugging
    eprintln!("Sending request to Stable Diffusion API: {}", json_data);

    // Create POST request with headers
    let mut req = Request::post(&api_url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(json_data.as_bytes().into_body())
        .map_err(|e| e.to_string())?;

    // Add API key if provided
    if !api_key.is_empty() {
        req = Request::post(&api_url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", &format!("Bearer {}", api_key))
            .body(json_data.as_bytes().into_body())
            .map_err(|e| e.to_string())?;
    }

    // Make request
    let mut response =
        Client::new().send(req).await.map_err(|e| format!("Request failed: {}", e))?;

    // Check for success
    if response.status() != 200 {
        let mut error_body = Vec::new();
        response
            .body_mut()
            .read_to_end(&mut error_body)
            .await
            .map_err(|e| format!("Failed to read error response: {}", e))?;

        let error_text = String::from_utf8_lossy(&error_body);
        return Err(format!("API error: status {} - {}", response.status(), error_text));
    }

    // Read response body
    let mut body_buf = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body_buf)
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    // Try parsing with our structured response first
    let image_base64 = match serde_json::from_slice::<StableDiffusionResponse>(&body_buf) {
        Ok(sd_response) => {
            // Return the first image (base64 encoded)
            sd_response.images.first().cloned().ok_or_else(|| "No image generated".to_string())?
        }
        Err(e) => {
            // Fallback: try parsing just to get the images array
            eprintln!("Error parsing full response: {}", e);

            // Try parsing as generic JSON Value
            let json_value: serde_json::Value = serde_json::from_slice(&body_buf)
                .map_err(|e| format!("Failed to parse response as JSON: {}", e))?;

            // Extract images array from the generic JSON
            if let Some(images) = json_value.get("images").and_then(|i| i.as_array()) {
                if let Some(first_image) = images.first().and_then(|i| i.as_str()) {
                    first_image.to_string()
                } else {
                    return Err("Could not extract image from response".to_string());
                }
            } else {
                return Err("No images array found in response".to_string());
            }
        }
    };

    // Format as data URI with proper MIME type
    // Stable Diffusion typically returns PNG images
    Ok(format!("data:image/png;base64,{}", image_base64))
}
