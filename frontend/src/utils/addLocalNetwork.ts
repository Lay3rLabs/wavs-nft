/**
 * Adds the local Anvil/Hardhat network to MetaMask
 */
export async function addLocalNetwork() {
  if (!window.ethereum) {
    console.error('MetaMask is not installed');
    return;
  }

  try {
    // Add the local Anvil network to MetaMask
    await window.ethereum.request({
      method: 'wallet_addEthereumChain',
      params: [
        {
          chainId: '0x539', // 1337 in hex
          chainName: 'Local Blockchain',
          nativeCurrency: {
            name: 'Ethereum',
            symbol: 'ETH',
            decimals: 18,
          },
          rpcUrls: ['http://localhost:8545'],
          blockExplorerUrls: [],
        },
      ],
    });
    
    console.log('Successfully added Local Blockchain network to MetaMask');
    
    // Switch to the local network
    await window.ethereum.request({
      method: 'wallet_switchEthereumChain',
      params: [{ chainId: '0x539' }], // 1337 in hex
    });
    
    console.log('Successfully switched to Local Blockchain network');
  } catch (error: any) {
    // Check if user rejected the request
    if (error.code === 4001) {
      console.error('User rejected the request to add the network');
    } else {
      console.error('Error adding/switching network:', error);
    }
  }
}