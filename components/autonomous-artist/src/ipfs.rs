use anyhow::Result;
use cid::Cid;
use multihash::{Code, Hasher, MultihashDigest, Sha2_256};
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
async fn upload_to_ipfs(file_path: &str, ipfs_url: &str) -> Result<IpfsResponse> {
    let api_key = std::env::var("WAVS_ENV_LIGHTHOUSE_API_KEY")
        .map_err(|e| anyhow::anyhow!("Failed to get API key: {}", e))?;

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

/// Uploads JSON data directly to IPFS and returns the CID
pub async fn upload_json_to_ipfs(json_data: &str, ipfs_url: &str) -> Result<String> {
    // Create a temporary file to store the JSON data
    let temp_filename = format!(
        "temp_json_{}.json",
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()
    );
    let temp_path = format!("/tmp/{}", temp_filename);

    // Write JSON to temporary file
    let mut file = File::create(&temp_path)?;
    file.write_all(json_data.as_bytes())?;

    // Upload the file
    let response = upload_to_ipfs(&temp_path, ipfs_url).await?;

    // Clean up the temporary file
    delete_file(&temp_path)?;

    // Return the CID (hash)
    Ok(response.hash)
}

/// Uploads an image to IPFS and returns the CID
pub async fn upload_image_to_ipfs(
    image_data: &[u8],
    filename: &str,
    ipfs_url: &str,
) -> Result<String> {
    // Create a temporary file to store the image data
    let temp_path = format!("/tmp/{}", filename);

    // Write image data to temporary file
    let mut file = File::create(&temp_path)?;
    file.write_all(image_data)?;

    // Upload the file
    let response = upload_to_ipfs(&temp_path, ipfs_url).await?;

    // Clean up the temporary file
    delete_file(&temp_path)?;

    // Return the CID (hash)
    Ok(response.hash)
}

/// Calculate the CID for file content without uploading
/// Uses CIDv1 with raw codec and SHA-256 multihash
pub fn calculate_cid(data: &[u8]) -> Result<String> {
    // Create a multihash using SHA-256
    let hash = Code::Sha2_256.digest(data);

    // Create a CIDv1 with raw codec (0x55)
    let cid = Cid::new_v1(0x55, hash);

    // Return the CID as a string
    Ok(cid.to_string())
}

/// Delete a file from the filesystem
pub fn delete_file(file_path: &str) -> Result<()> {
    std::fs::remove_file(file_path)?;
    println!("File deleted successfully: {}", file_path);
    Ok(())
}

/// Get IPFS URL from CID
pub fn get_ipfs_url(cid: &str) -> String {
    format!("ipfs://{}", cid)
}

/// Get HTTP gateway URL from CID
pub fn get_ipfs_gateway_url(cid: &str, gateway: &str) -> String {
    format!("{}/ipfs/{}", gateway.trim_end_matches('/'), cid)
}

/// Uploads NFT content (metadata and/or image) to IPFS
/// Returns the IPFS URI (ipfs://CID) for the content
pub async fn upload_nft_content(
    content_type: &str,
    content: &[u8],
    ipfs_url: &str,
) -> Result<String> {
    // Determine if this is JSON metadata or an image
    let (filename, cid) = if content_type.contains("json") || content_type == "application/json" {
        // It's JSON metadata
        let json_str = std::str::from_utf8(content)
            .map_err(|e| anyhow::anyhow!("Failed to convert JSON bytes to string: {}", e))?;

        let filename = format!(
            "nft_metadata_{}.json",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs()
        );

        // Upload the JSON
        let cid = upload_json_to_ipfs(json_str, ipfs_url).await?;
        (filename, cid)
    } else {
        // It's an image or other binary content
        let extension = match content_type {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/gif" => "gif",
            "image/svg+xml" => "svg",
            _ => "bin", // Default extension for unknown types
        };

        let filename = format!(
            "nft_image_{}.{}",
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs(),
            extension
        );

        // Upload the image
        let cid = upload_image_to_ipfs(content, &filename, ipfs_url).await?;
        (filename, cid)
    };

    // Log the upload
    println!("Uploaded {} to IPFS with CID: {}", filename, cid);

    // Return IPFS URI
    Ok(get_ipfs_url(&cid))
}
