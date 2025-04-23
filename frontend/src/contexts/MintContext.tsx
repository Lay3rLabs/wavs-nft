import React, {
  createContext,
  useState,
  useContext,
  useEffect,
  ReactNode,
} from "react";
import { useAccount, usePublicClient, useWalletClient } from "wagmi";
import { ethers } from "ethers";
import WavsMinterABI from "../abis/WavsMinter.json";
import WavsNftABI from "../abis/WavsNft.json";

// You would replace these with your deployed contract addresses
const MINTER_CONTRACT_ADDRESS = "0x809d550fca64d94bd9f66e60752a544199cfac3d"; // Replace with actual address
const NFT_CONTRACT_ADDRESS = "0x36c02da8a0983159322a80ffe9f24b1acff8b570"; // Replace with actual address

// Default mint price as a fallback when contract call fails
const DEFAULT_MINT_PRICE = "0.1"; // in ETH

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
  mintPrice: "0",
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
  const [mintPrice, setMintPrice] = useState<string>("0");
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
    const provider = new ethers.providers.JsonRpcProvider(
      "http://localhost:8545"
    );
    return new ethers.Contract(
      MINTER_CONTRACT_ADDRESS,
      WavsMinterABI,
      provider
    );
  };

  const getNftContract = () => {
    if (!publicClient) return null;
    // For ethers v5 compatibility with wagmi
    const provider = new ethers.providers.JsonRpcProvider(
      "http://localhost:8545"
    );
    return new ethers.Contract(NFT_CONTRACT_ADDRESS, WavsNftABI, provider);
  };

  // Get the mint price from the contract
  const loadMintPrice = async () => {
    try {
      setLoadingMintPrice(true);
      const minterContract = getMinterContract();
      if (!minterContract) return;

      try {
        // First check if the contract exists by getting its code
        const provider = new ethers.providers.JsonRpcProvider("http://localhost:8545");
        const code = await provider.getCode(MINTER_CONTRACT_ADDRESS);
        
        if (code === '0x') {
          console.error(`Contract does not exist at ${MINTER_CONTRACT_ADDRESS}`);
          setMintPrice(DEFAULT_MINT_PRICE);
          return;
        }
        
        // If the contract exists, try to call mintPrice()
        const price = await minterContract.mintPrice();
        setMintPrice(ethers.utils.formatEther(price));
      } catch (error) {
        console.error(
          "Error fetching mint price from contract, using default:",
          error
        );
        // Use default mint price as fallback
        setMintPrice(DEFAULT_MINT_PRICE);
      }
    } catch (error) {
      console.error("Error loading mint price:", error);
      // Use default mint price as fallback
      setMintPrice(DEFAULT_MINT_PRICE);
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
      console.error("Error loading user data:", error);
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
        const recent = parsedMints.filter(
          (mint: PendingMint) =>
            Date.now() - mint.timestamp < 24 * 60 * 60 * 1000
        );
        setPendingMints(recent);
        localStorage.setItem(`pendingMints-${address}`, JSON.stringify(recent));
      }
    } catch (error) {
      console.error("Error loading pending mints:", error);
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
          const isIpfs = tokenURI.startsWith("ipfs://");
          const metadataUrl = isIpfs
            ? tokenURI.replace("ipfs://", "https://gateway.lighthouse.storage/ipfs/")
            : tokenURI;

          const response = await fetch(metadataUrl);
          metadata = await response.json();

          if (metadata.image) {
            imageUrl = metadata.image.startsWith("ipfs://")
              ? metadata.image.replace("ipfs://", "https://gateway.lighthouse.storage/ipfs/")
              : metadata.image;
          }
        } catch (error) {
          console.error(`Error fetching metadata for token ${tokenId}:`, error);
        }

        nfts.push({
          tokenId: tokenId.toString(),
          tokenURI,
          imageUrl,
          metadata,
        });
      }

      setOwnedNfts(nfts);
    } catch (error) {
      console.error("Error loading owned NFTs:", error);
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
      
      // Check if the contract exists
      const jsonRpcProvider = new ethers.providers.JsonRpcProvider("http://localhost:8545");
      const bytecode = await jsonRpcProvider.getCode(MINTER_CONTRACT_ADDRESS);
      if (bytecode === '0x') {
        throw new Error(`Contract does not exist at ${MINTER_CONTRACT_ADDRESS}. Please make sure the contracts are deployed to your local network.`);
      }

      // Create contract instance with ethers
      const contract = new ethers.Contract(
        MINTER_CONTRACT_ADDRESS,
        WavsMinterABI,
        signer
      );

      // Get the mint price from the contract
      let price;
      try {
        price = await contract.mintPrice();
      } catch (error) {
        console.error(
          "Error fetching mint price from contract, using default:",
          error
        );
        // Use default mint price as fallback
        price = ethers.utils.parseEther(DEFAULT_MINT_PRICE);
      }

      // Check wallet balance
      const balance = await provider.getBalance(address);
      if (balance.lt(price)) {
        const formatBalance = ethers.utils.formatEther(balance);
        const formatPrice = ethers.utils.formatEther(price);
        console.error(
          `Insufficient balance: ${formatBalance} ETH, needed: ${formatPrice} ETH (not including gas)`
        );

        // Get the current chain to provide specific guidance
        const chainId = publicClient.chain.id;
        const chainName = publicClient.chain.name.toLowerCase();
        let faucetInfo = "";

        // Add network-specific information
        if (
          chainId === 31337 || // Anvil default
          chainId === 1337 ||  // Ganache/Hardhat default
          chainName.includes("local") ||
          chainName.includes("anvil") ||
          chainName.includes("hardhat") ||
          chainName.includes("localhost")
        ) {
          // Local Anvil/Hardhat environment
          faucetInfo =
            "For Anvil/local node: Use `anvil --balance 10000` to increase starting balance or send ETH to your account with `cast send --value 1ether <your-address> --private-key <anvil-private-key>`";
        } else if (chainId === 1) {
          // Mainnet - no faucets
          faucetInfo = "You'll need to purchase ETH from an exchange.";
        } else if (chainId === 5) {
          // Goerli
          faucetInfo = "Get test ETH from goerlifaucet.com";
        } else if (chainId === 11155111) {
          // Sepolia
          faucetInfo = "Get test ETH from sepoliafaucet.com";
        } else if (chainId === 80001) {
          // Mumbai
          faucetInfo = "Get test MATIC from mumbai.polygonscan.com/faucet";
        } else {
          // Generic message for other networks
          faucetInfo =
            "Search for a faucet for your current network to get test tokens.";
        }

        throw new Error(
          `Insufficient balance: You have ${formatBalance} ETH, but need at least ${formatPrice} ETH plus gas fees. ${faucetInfo}`
        );
      }

      // Check if we're in a local/development environment for additional debugging
      const isLocalEnv = true; // Always show debugging in development
      console.log("Chain ID:", publicClient.chain.id);
      console.log("Chain Name:", publicClient.chain.name);

      // For local environments, log more debugging info
      if (isLocalEnv) {
        console.log("Local environment detected, logging debug info:");
        console.log("Chain ID:", publicClient.chain.id);
        console.log("Chain Name:", publicClient.chain.name);
        console.log("Minter Contract Address:", MINTER_CONTRACT_ADDRESS);
        console.log("NFT Contract Address:", NFT_CONTRACT_ADDRESS);
        console.log("Connected Address:", address);
        console.log("Mint Price (wei):", price.toString());
        console.log(
          "Balance (wei):",
          (await provider.getBalance(address)).toString()
        );

        try {
          // Try to read contract bytecode to check if it exists
          const minterBytecode = await provider.getCode(MINTER_CONTRACT_ADDRESS);
          const nftBytecode = await provider.getCode(NFT_CONTRACT_ADDRESS);
          
          console.log("Minter Contract exists?", minterBytecode !== "0x");
          console.log("NFT Contract exists?", nftBytecode !== "0x");
          
          if (minterBytecode === "0x") {
            console.error(
              "⚠️ MINTER CONTRACT DOES NOT EXIST at address:",
              MINTER_CONTRACT_ADDRESS
            );
            console.log("Deploy the contract with: forge script script/Deploy.s.sol:Deploy --rpc-url http://localhost:8545 --broadcast");
          }
          
          if (nftBytecode === "0x") {
            console.error(
              "⚠️ NFT CONTRACT DOES NOT EXIST at address:",
              NFT_CONTRACT_ADDRESS
            );
          }
          
          // Show a way to deploy with the specific contract addresses
          console.log("\nTo deploy the contracts to these exact addresses, run the following commands:");
          console.log(`1. Start anvil with specific accounts: 
           anvil --accounts 2 --balance 10000 --chain-id 1337`);
          console.log(`2. Deploy the contracts with the specific addresses:
           forge script script/Deploy.s.sol:Deploy --rpc-url http://localhost:8545 --broadcast`);
          console.log("\nAfter deployment, check if the contracts were deployed to the expected addresses.");
        } catch (e) {
          console.error("Error checking contract bytecode:", e);
        }
      }

      // Execute the transaction
      const tx = await contract.triggerMint(prompt, {
        value: price,
        // Adding explicit gasLimit to prevent estimation errors
        gasLimit: 300000,
      });

      // Wait for the transaction to be mined
      const receipt = await tx.wait();

      // Find the WavsNftTrigger event in the logs
      const event = receipt.events?.find((e) => e.event === "WavsNftTrigger");
      if (!event) return null;

      const triggerId = event.args.triggerId.toString();

      // Store pending mint in local storage
      const newPendingMint = {
        triggerId,
        prompt,
        timestamp: Date.now(),
      };

      const updatedPendingMints = [...pendingMints, newPendingMint];
      setPendingMints(updatedPendingMints);
      localStorage.setItem(
        `pendingMints-${address}`,
        JSON.stringify(updatedPendingMints)
      );

      return triggerId;
    } catch (error) {
      console.error("Error triggering mint:", error);

      // Check for specific error types and provide better feedback
      if (error instanceof Error) {
        // For insufficient balance errors, propagate our custom error
        if (error.message.includes("Insufficient balance")) {
          throw error;
        }

        // For contract not found errors
        if (
          error.message.includes("call revert exception") &&
          error.message.includes('method="mintPrice()"')
        ) {
          console.error("Contract method not available. This usually means:");
          console.error(
            "1. The contract doesn't exist at the specified address"
          );
          console.error(
            "2. The contract at that address doesn't have the expected functions"
          );
          console.error("3. You might be connected to the wrong network");

          const errorMsg = `Contract error: The mintPrice() function is not available. Make sure the contract is deployed at ${MINTER_CONTRACT_ADDRESS} on your current network.`;
          throw new Error(errorMsg);
        }
      }

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
      const matchingMint = pendingMints.find(
        (m) => m.triggerId === triggerIdStr
      );
      if (!matchingMint) return;

      // Remove this mint from pending
      const updatedPendingMints = pendingMints.filter(
        (m) => m.triggerId !== triggerIdStr
      );
      setPendingMints(updatedPendingMints);
      localStorage.setItem(
        `pendingMints-${address}`,
        JSON.stringify(updatedPendingMints)
      );

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
    refreshNfts,
  };

  return <MintContext.Provider value={value}>{children}</MintContext.Provider>;
};
