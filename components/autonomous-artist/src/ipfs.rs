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

    eprintln!("Uploading file to IPFS: {}", file_path);

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

        // Log the raw response for debugging
        let response_str = std::str::from_utf8(&body_buf)
            .map_err(|e| anyhow::anyhow!("Failed to convert response to string: {}", e))?;
        eprintln!("IPFS API Response: {}", response_str);

        // Parse using Lighthouse's response format (capitalized fields)
        let lighthouse_response: LighthouseResponse = match serde_json::from_slice(&body_buf) {
            Ok(resp) => resp,
            Err(e) => {
                // Simple fallback - just look for the hash in the response text
                eprintln!("Failed to parse response: {}", e);

                if let Some(start) = response_str.find("\"Hash\":\"") {
                    if let Some(end) = response_str[start + 8..].find("\"") {
                        let hash = &response_str[start + 8..start + 8 + end];
                        return Ok(IpfsResponse {
                            hash: hash.to_string(),
                            name: Path::new(file_path)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            size: "0".to_string(),
                        });
                    }
                }

                // If that fails too, try lowercase
                if let Some(start) = response_str.find("\"hash\":\"") {
                    if let Some(end) = response_str[start + 8..].find("\"") {
                        let hash = &response_str[start + 8..start + 8 + end];
                        return Ok(IpfsResponse {
                            hash: hash.to_string(),
                            name: Path::new(file_path)
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string(),
                            size: "0".to_string(),
                        });
                    }
                }

                return Err(anyhow::anyhow!(
                    "Could not extract hash from response: {}",
                    response_str
                ));
            }
        };

        // Convert to our standard response format
        Ok(IpfsResponse {
            hash: lighthouse_response.Hash,
            name: lighthouse_response.Name,
            size: lighthouse_response.Size,
        })
    } else {
        let mut body_buf = Vec::new();
        response.body_mut().read_to_end(&mut body_buf).await?;
        let error_body = std::str::from_utf8(&body_buf).unwrap_or("unable to read error body");
        Err(anyhow::anyhow!(
            "Failed to upload to IPFS. Status: {:?}, Body: {}",
            response.status(),
            error_body
        ))
    }
}

#[derive(Debug, Deserialize)]
struct LighthouseResponse {
    Hash: String,
    #[serde(default)]
    Name: String,
    #[serde(default)]
    Size: String,
}

#[derive(Debug, Deserialize)]
struct IpfsResponse {
    #[serde(default)]
    name: String,
    hash: String,
    #[serde(default)]
    size: String,
}

/// Uploads JSON data directly to IPFS and returns the CID
pub async fn upload_json_to_ipfs(json_data: &str, ipfs_url: &str) -> Result<String> {
    // Create a temporary file to store the JSON data
    let filename = "nft_metadata.json".to_string();
    let temp_path = format!("/tmp/{}", filename);

    eprint!("Temp path {}", temp_path);

    // Ensure the /tmp directory exists
    std::fs::create_dir_all("/tmp")
        .map_err(|e| anyhow::anyhow!("Failed to create /tmp directory: {}", e))?;

    // Write JSON to temporary file
    let mut file = File::create(&temp_path)?;
    file.write_all(json_data.as_bytes())?;

    // Upload the file
    let response = upload_to_ipfs(&temp_path, ipfs_url).await?;

    // Clean up the temporary file
    delete_file(&temp_path)?;

    // Return the IPFS URI
    Ok(get_ipfs_url(&response.hash, Some(&filename)))
}

/// Uploads an image to IPFS and returns the CID
pub async fn upload_image_to_ipfs(
    image_data: &[u8],
    filename: &str,
    ipfs_url: &str,
) -> Result<String> {
    // Create a temporary file to store the image data
    let temp_path = format!("/tmp/{}", filename);

    // Ensure the /tmp directory exists
    std::fs::create_dir_all("/tmp")
        .map_err(|e| anyhow::anyhow!("Failed to create /tmp directory: {}", e))?;

    // Write image data to temporary file
    let mut file = File::create(&temp_path)?;
    file.write_all(image_data)?;

    // Upload the file
    let response = upload_to_ipfs(&temp_path, ipfs_url).await?;

    // Clean up the temporary file
    delete_file(&temp_path)?;

    // Return the IPFS URI
    Ok(get_ipfs_url(&response.hash, Some(filename)))
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
/// If filename is provided, constructs a URL that points to a file within a directory
pub fn get_ipfs_url(cid: &str, filename: Option<&str>) -> String {
    match filename {
        Some(name) => format!("ipfs://{}/{}", cid, name),
        None => format!("ipfs://{}", cid),
    }
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
    let ipfs_uri = if content_type.contains("json") || content_type == "application/json" {
        // It's JSON metadata
        let json_str = std::str::from_utf8(content)
            .map_err(|e| anyhow::anyhow!("Failed to convert JSON bytes to string: {}", e))?;

        // Upload the JSON and return the IPFS URI
        upload_json_to_ipfs(json_str, ipfs_url).await?
    } else {
        // It's an image or other binary content
        let extension = match content_type {
            "image/png" => "png",
            "image/jpeg" => "jpg",
            "image/gif" => "gif",
            "image/svg+xml" => "svg",
            _ => "bin", // Default extension for unknown types
        };

        let filename = format!("nft_image.{}", extension);

        // Upload the image and return the IPFS URI
        upload_image_to_ipfs(content, &filename, ipfs_url).await?
    };

    // Log the upload
    println!("Uploaded to IPFS with URI: {}", ipfs_uri);

    // Return IPFS URI with filename to handle directory structure
    Ok(ipfs_uri)
}
