import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './index.css';
import { WagmiConfig } from 'wagmi';
import {
  RainbowKitProvider,
  getDefaultWallets,
  connectorsForWallets,
} from '@rainbow-me/rainbowkit';
import '@rainbow-me/rainbowkit/styles.css';
import { configureChains, createConfig } from 'wagmi';
import { mainnet, sepolia, localhost } from 'wagmi/chains';
import { publicProvider } from 'wagmi/providers/public';
import { jsonRpcProvider } from 'wagmi/providers/jsonRpc';
import { Toaster } from 'react-hot-toast';

const { chains, publicClient, webSocketPublicClient } = configureChains(
  [mainnet, sepolia, localhost],
  [
    jsonRpcProvider({
      rpc: () => ({
        http: 'http://localhost:8545',
      }),
    }),
    publicProvider()
  ]
);

const { wallets } = getDefaultWallets({
  appName: 'Wavs NFT Generator',
  projectId: 'YOUR_WALLETCONNECT_PROJECT_ID', // Replace with your WalletConnect project ID in production
  chains,
});

const connectors = connectorsForWallets(wallets);

const wagmiConfig = createConfig({
  autoConnect: true,
  connectors,
  publicClient,
  webSocketPublicClient,
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <WagmiConfig config={wagmiConfig}>
      <RainbowKitProvider chains={chains}>
        <App />
        <Toaster position="top-right" />
      </RainbowKitProvider>
    </WagmiConfig>
  </React.StrictMode>
);