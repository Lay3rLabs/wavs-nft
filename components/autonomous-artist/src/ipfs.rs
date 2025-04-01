use anyhow::Result;
use serde::Deserialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use wavs_wasi_chain::http::{fetch_bytes, http_request_get};
use wstd::http::{IntoBody, Request};
use wstd::io::AsyncRead;

/// downloads a file from a given URL and saves it to the specified local path
async fn download_file(url: &str, file_name: &str) -> Result<String> {
    let request = http_request_get(url)?;
    let file_bytes = fetch_bytes(request).await?;

    let full_path = format!("/tmp/{}", file_name);
    let path = Path::new(&full_path);

    let mut file = File::create(path)?;
    file.write_all(&file_bytes)?;

    println!("File downloaded successfully to {}", full_path);
    Ok(full_path)
}

/// Uploads a file using multipart request to IPFS
async fn upload_to_ipfs(file_path: &str, ipfs_url: &str, api_key: &str) -> Result<IpfsResponse> {
    let mut file = File::open(file_path)?;
    let mut file_bytes = Vec::new();
    file.read_to_end(&mut file_bytes)?;

    // define multipart request boundary
    let boundary = "----RustBoundary";

    // construct the body
    let body = format!(
        "--{}\r\n\
        Content-Disposition: form-data; name=\"file\"; filename=\"{}\"\r\n\
        Content-Type: application/octet-stream\r\n\r\n",
        boundary, file_path
    );

    let mut request_body = body.into_bytes();
    request_body.extend_from_slice(&file_bytes);
    request_body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

    let request = Request::post(ipfs_url)
        .header("Authorization", &format!("Bearer {}", api_key))
        .header("Content-Type", &format!("multipart/form-data; boundary={}", boundary))
        .body(request_body.into_body())?;

    let mut response = wstd::http::Client::new().send(request).await?;

    if response.status().is_success() {
        let mut body_buf = Vec::new();
        response.body_mut().read_to_end(&mut body_buf).await?;
        let ipfs_response: IpfsResponse = serde_json::from_slice(&body_buf)?;
        Ok(ipfs_response)
    } else {
        Err(anyhow::anyhow!("Failed to upload to IPFS. Status: {:?}", response.status()))
    }
}

#[derive(Debug, Deserialize)]
struct IpfsResponse {
    name: String,
    hash: String,
    size: String,
}
