# Autonomous Artist

Status: Highly experimental

## Overview

The Autonomous Artist component is designed to create NFTs with AI-generated content. It integrates with a local Ollama instance to generate content based on user prompts and creates corresponding NFTs on the blockchain.

## Run Ollama locally

```bash
ollama run llama3.1
```

## How It Works

### The Trigger

The component listens for `AvsMintTrigger` events containing:
- Sender's address
- Prompt for AI generation
- Trigger ID
- Trigger type (MINT or UPDATE)

### AI Generation

When triggered, the component:
1. Processes the provided prompt
2. Sends it to the local Ollama instance with deterministic generation parameters

### NFT Creation

The component creates NFT metadata:
- Name
- Description: the AI-generated response
- Image: the IPFS URI
- Attributes: the original prompt

### Prepare for minting

Finally, the component serializes the metadata to JSON and converts it to a data URI in `WavsMintResult`, which contains:
  - the Trigger ID
  - Recipient (sender's address)
  - Token URI containing the metadata

The data is then ready to be minted as an NFT by the [`WavsNft.sol`](/src/contracts/WavsNft.sol) contract.
