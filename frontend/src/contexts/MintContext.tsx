import React, { createContext, useState, useContext, useEffect, ReactNode } from 'react';
import { useAccount, usePublicClient, useWalletClient } from 'wagmi';
import { ethers } from 'ethers';
import WavsMinterABI from '../abis/WavsMinter.json';
import WavsNftABI from '../abis/WavsNft.json';

// You would replace these with your deployed contract addresses
const MINTER_CONTRACT_ADDRESS = '0x0000000000000000000000000000000000000000'; // Replace with actual address
const NFT_CONTRACT_ADDRESS = '0x0000000000000000000000000000000000000000';   // Replace with actual address

interface MintContextType {
  mintPrice: string;
  pendingMints: PendingMint[];
  ownedNfts: OwnedNft[];
  loadingMintPrice: boolean;
  loadingNfts: boolean;
  triggerMint: (prompt: string) => Promise<string | null>;
  refreshNfts: () => Promise<void>;
}

interface PendingMint {
  triggerId: string;
  prompt: string;
  timestamp: number;
}

interface OwnedNft {
  tokenId: string;
  tokenURI: string;
  imageUrl: string | null;
  metadata: any;
}

export const MintContext = createContext<MintContextType>({
  mintPrice: '0',
  pendingMints: [],
  ownedNfts: [],
  loadingMintPrice: false,
  loadingNfts: false,
  triggerMint: async () => null,
  refreshNfts: async () => {},
});

export const useMint = () => useContext(MintContext);

interface MintProviderProps {
  children: ReactNode;
}

export const MintProvider: React.FC<MintProviderProps> = ({ children }) => {
  const [mintPrice, setMintPrice] = useState<string>('0');
  const [pendingMints, setPendingMints] = useState<PendingMint[]>([]);
  const [ownedNfts, setOwnedNfts] = useState<OwnedNft[]>([]);
  const [loadingMintPrice, setLoadingMintPrice] = useState(false);
  const [loadingNfts, setLoadingNfts] = useState(false);
  
  const { address, isConnected } = useAccount();
  const publicClient = usePublicClient();
  const { data: walletClient } = useWalletClient();

  // Create contract instances
  const getMinterContract = () => {
    if (!publicClient) return null;
    // For ethers v5 compatibility with wagmi
    const provider = new ethers.providers.JsonRpcProvider(publicClient.chain.rpcUrls.default.http[0]);
    return new ethers.Contract(MINTER_CONTRACT_ADDRESS, WavsMinterABI, provider);
  };

  const getNftContract = () => {
    if (!publicClient) return null;
    // For ethers v5 compatibility with wagmi
    const provider = new ethers.providers.JsonRpcProvider(publicClient.chain.rpcUrls.default.http[0]);
    return new ethers.Contract(NFT_CONTRACT_ADDRESS, WavsNftABI, provider);
  };

  // Load mint price from the contract
  const loadMintPrice = async () => {
    try {
      setLoadingMintPrice(true);
      const minterContract = getMinterContract();
      if (!minterContract) return;
      
      const price = await minterContract.mintPrice();
      setMintPrice(ethers.utils.formatEther(price));
    } catch (error) {
      console.error('Error loading mint price:', error);
    } finally {
      setLoadingMintPrice(false);
    }
  };

  // Load pending mints and owned NFTs
  const loadUserData = async () => {
    if (!isConnected || !address) {
      setPendingMints([]);
      setOwnedNfts([]);
      return;
    }

    try {
      setLoadingNfts(true);
      await Promise.all([loadPendingMints(), loadOwnedNfts()]);
    } catch (error) {
      console.error('Error loading user data:', error);
    } finally {
      setLoadingNfts(false);
    }
  };

  // Load pending mints that haven't been fulfilled yet
  const loadPendingMints = async () => {
    if (!address) return;
    
    try {
      // This would need to be implemented based on your backend/indexer
      // For demo purposes, we'll just use local storage
      const storedMints = localStorage.getItem(`pendingMints-${address}`);
      if (storedMints) {
        const parsedMints = JSON.parse(storedMints);
        // Filter out any that are too old (over 24 hours)
        const recent = parsedMints.filter((mint: PendingMint) => 
          Date.now() - mint.timestamp < 24 * 60 * 60 * 1000
        );
        setPendingMints(recent);
        localStorage.setItem(`pendingMints-${address}`, JSON.stringify(recent));
      }
    } catch (error) {
      console.error('Error loading pending mints:', error);
    }
  };

  // Load user's owned NFTs
  const loadOwnedNfts = async () => {
    if (!address) return;

    try {
      const nftContract = getNftContract();
      if (!nftContract) return;
      
      const balance = await nftContract.balanceOf(address);
      const tokenCount = balance.toNumber();
      
      const nfts: OwnedNft[] = [];
      
      for (let i = 0; i < tokenCount; i++) {
        const tokenId = await nftContract.tokenOfOwnerByIndex(address, i);
        const tokenURI = await nftContract.tokenURI(tokenId);
        
        let metadata = null;
        let imageUrl = null;
        
        try {
          // Assume tokenURI is either IPFS or HTTP URL
          const isIpfs = tokenURI.startsWith('ipfs://');
          const metadataUrl = isIpfs 
            ? tokenURI.replace('ipfs://', 'https://ipfs.io/ipfs/') 
            : tokenURI;
            
          const response = await fetch(metadataUrl);
          metadata = await response.json();
          
          if (metadata.image) {
            imageUrl = metadata.image.startsWith('ipfs://')
              ? metadata.image.replace('ipfs://', 'https://ipfs.io/ipfs/')
              : metadata.image;
          }
        } catch (error) {
          console.error(`Error fetching metadata for token ${tokenId}:`, error);
        }
        
        nfts.push({
          tokenId: tokenId.toString(),
          tokenURI,
          imageUrl,
          metadata
        });
      }
      
      setOwnedNfts(nfts);
    } catch (error) {
      console.error('Error loading owned NFTs:', error);
    }
  };

  // Trigger a new mint
  const triggerMint = async (prompt: string): Promise<string | null> => {
    if (!address || !isConnected || !walletClient || !publicClient) return null;
    
    try {
      // Create ethers signer
      const provider = new ethers.providers.Web3Provider({
        request: walletClient.request.bind(walletClient),
      } as any);
      const signer = provider.getSigner();
      
      // Create contract instance with ethers
      const contract = new ethers.Contract(
        MINTER_CONTRACT_ADDRESS,
        WavsMinterABI,
        signer
      );
      
      // Get the mint price from the contract
      const price = await contract.mintPrice();
      
      // Execute the transaction
      const tx = await contract.triggerMint(prompt, {
        value: price
      });
      
      // Wait for the transaction to be mined
      const receipt = await tx.wait();
      
      // Find the WavsNftTrigger event in the logs
      const event = receipt.events?.find(e => e.event === 'WavsNftTrigger');
      if (!event) return null;
      
      const triggerId = event.args.triggerId.toString();
      
      // Store pending mint in local storage
      const newPendingMint = {
        triggerId,
        prompt,
        timestamp: Date.now()
      };
      
      const updatedPendingMints = [...pendingMints, newPendingMint];
      setPendingMints(updatedPendingMints);
      localStorage.setItem(`pendingMints-${address}`, JSON.stringify(updatedPendingMints));
      
      return triggerId;
    } catch (error) {
      console.error('Error triggering mint:', error);
      return null;
    }
  };

  // Refresh NFTs
  const refreshNfts = async (): Promise<void> => {
    await loadUserData();
  };

  // Set up event listeners for MintFulfilled events
  useEffect(() => {
    const minterContract = getMinterContract();
    if (!minterContract || !address || !publicClient) return;
    
    const mintFulfilledFilter = minterContract.filters.MintFulfilled();
    
    const handleMintFulfilled = async (triggerId: ethers.BigNumber) => {
      const triggerIdStr = triggerId.toString();
      
      // Check if this triggerId matches any of our pending mints
      const matchingMint = pendingMints.find(m => m.triggerId === triggerIdStr);
      if (!matchingMint) return;
      
      // Remove this mint from pending
      const updatedPendingMints = pendingMints.filter(m => m.triggerId !== triggerIdStr);
      setPendingMints(updatedPendingMints);
      localStorage.setItem(`pendingMints-${address}`, JSON.stringify(updatedPendingMints));
      
      // Refresh owned NFTs to include the new one
      await loadOwnedNfts();
    };
    
    minterContract.on(mintFulfilledFilter, handleMintFulfilled);
    
    return () => {
      minterContract.off(mintFulfilledFilter, handleMintFulfilled);
    };
  }, [pendingMints, address, publicClient]);
  
  // Listen for NFT mint events to refresh owned NFTs
  useEffect(() => {
    const nftContract = getNftContract();
    if (!nftContract || !address || !publicClient) return;
    
    const wavsNftMintFilter = nftContract.filters.WavsNftMint(address);
    
    const handleNftMint = async () => {
      await loadOwnedNfts();
    };
    
    nftContract.on(wavsNftMintFilter, handleNftMint);
    
    return () => {
      nftContract.off(wavsNftMintFilter, handleNftMint);
    };
  }, [address, publicClient]);

  // Load initial data
  useEffect(() => {
    loadMintPrice();
  }, []);

  useEffect(() => {
    if (isConnected) {
      loadUserData();
    }
  }, [isConnected, address]);

  const value = {
    mintPrice,
    pendingMints,
    ownedNfts,
    loadingMintPrice,
    loadingNfts,
    triggerMint,
    refreshNfts
  };

  return (
    <MintContext.Provider value={value}>
      {children}
    </MintContext.Provider>
  );
};