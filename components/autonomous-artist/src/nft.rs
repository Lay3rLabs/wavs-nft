use serde::Serialize;

// NFT Metadata structure
#[derive(Serialize, Debug)]
pub struct NFTMetadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Debug)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
