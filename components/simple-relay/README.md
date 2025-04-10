# Simple Relay

The Simple Relay component listens for NFT minting events and relays the completion status back to the minter contract to mark the receipt as fulfilled.

This component completes the NFT minting cycle:

1.  `WavsNft` contract mints an NFT and emits an `NFTMinted` event which triggers the component.
2.  Simple Relay receives the event and extracts the `triggerId`.
3.  The `triggerId` is used to mark the corresponding receipt as fulfilled in the `WavsMinter` contract.
