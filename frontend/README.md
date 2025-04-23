# WAVS NFT Generator Frontend

This is the frontend application for the WAVS NFT Generator project. It provides a user interface for minting AI-generated NFTs using the WAVS service.

## Features

- Connect wallet using RainbowKit
- Mint NFTs by providing text prompts
- View pending mints with loading indicators
- Display minted NFTs in a gallery
- Toast notifications for success and error messages
- Responsive design with TailwindCSS

## Getting Started

1. First, make sure you have the contract addresses:

   Edit the `MINTER_CONTRACT_ADDRESS` and `NFT_CONTRACT_ADDRESS` in `src/contexts/MintContext.tsx` to match your deployed contracts.

2. Install dependencies:

   ```bash
   yarn install
   ```

3. Start the development server:

   ```bash
   yarn dev
   ```

4. Open [http://localhost:3000](http://localhost:3000) with your browser to see the result.

## Building for Production

To build the application for production:

```bash
yarn build
```

The build output will be in the `dist` directory.

## Project Structure

- `/src/components`: React components
- `/src/contexts`: Context providers for state management
- `/src/abis`: Contract ABIs
- `/src/hooks`: Custom React hooks
- `/public`: Static assets

## Technologies Used

- React
- TypeScript
- RainbowKit for wallet connection
- Ethers.js for blockchain interaction
- TailwindCSS for styling
- React Hot Toast for notifications
- Vite for build tooling