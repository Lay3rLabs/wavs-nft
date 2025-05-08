/**
 * Detects the local Anvil/Hardhat network chain ID and adds it to MetaMask
 */
export async function addLocalNetwork() {
  if (!window.ethereum) {
    console.error('MetaMask is not installed');
    return;
  }

  try {
    // First, detect the actual chain ID from our local node
    const response = await fetch('http://localhost:8545', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        jsonrpc: '2.0',
        method: 'eth_chainId',
        params: [],
        id: 1,
      }),
    });

    const data = await response.json();
    const actualChainId = data.result;
    
    console.log('Detected local blockchain chain ID:', actualChainId);
    
    // Convert to decimal for display
    const decimalChainId = parseInt(actualChainId, 16);
    console.log('Decimal chain ID:', decimalChainId);

    // Add the local network with the detected chain ID
    await window.ethereum.request({
      method: 'wallet_addEthereumChain',
      params: [
        {
          chainId: actualChainId,
          chainName: `Local Blockchain (${decimalChainId})`,
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
    
    console.log(`Successfully added Local Blockchain network (${actualChainId}) to MetaMask`);
    
    // Switch to the local network
    await window.ethereum.request({
      method: 'wallet_switchEthereumChain',
      params: [{ chainId: actualChainId }],
    });
    
    console.log(`Successfully switched to Local Blockchain network (${actualChainId})`);
    return actualChainId;
  } catch (error: any) {
    // Check if user rejected the request
    if (error.code === 4001) {
      console.error('User rejected the request to add the network');
    } else {
      console.error('Error adding/switching network:', error);
    }
    return null;
  }
}