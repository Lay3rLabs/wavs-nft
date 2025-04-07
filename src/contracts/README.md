# WAVS NFT Contracts

This directory contains the core smart contracts for the WAVS NFT system, which enables AI-generated NFT creation through the WAVS framework.

## Contracts

### WavsMinter.sol

The `WavsMinter` contract is responsible for managing the NFT mint process, including receiving payment, triggering the autonomous artist component. It tracks the mint and receipts and uses `handleSignedData` to verify the final mint which is relayed via the simple relay component.


### WavsNft.sol

The `WavsNft` contract implements the ERC721 standard for minting the NFT with the generated response and metadata from the autonomous artist and minter contract. In implements `handleSignedData` to process signed NFT data from the output of the autonomous artist component.

## Contract Interaction Flow

1. User initiates mint through `WavsMinter.triggerMint()`
2. Autonomous artist generates NFT data
3. `WavsNft` contract mints the NFT and emits `NFTMinted` event
4. Simple relay component processes the event
5. `WavsMinter` marks the receipt as fulfilled

### WavsMinter Events
- `AvsMintTrigger`: Emitted when a mint is requested
- `MintFulfilled`: Emitted when a mint is completed
- `MintPriceUpdated`: Emitted when mint price changes
- `FeesWithdrawn`: Emitted when fees are withdrawn

### WavsNft Events
- `NFTMinted`: Emitted when an NFT is successfully minted
