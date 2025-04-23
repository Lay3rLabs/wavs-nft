import React, { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { useMint } from '../contexts/MintContext';
import { ethers } from 'ethers';

const NFTDetailPage: React.FC = () => {
  const { tokenId } = useParams<{ tokenId: string }>();
  const { ownedNfts } = useMint();
  const navigate = useNavigate();
  const [nft, setNft] = useState<any>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Find the NFT with the matching tokenId
    const foundNft = ownedNfts.find((n) => n.tokenId === tokenId);
    
    if (foundNft) {
      setNft(foundNft);
    }
    
    setLoading(false);
  }, [tokenId, ownedNfts]);

  if (loading) {
    return (
      <div className="min-h-screen bg-dark-800 text-primary flex items-center justify-center">
        <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-primary"></div>
      </div>
    );
  }

  if (!nft) {
    return (
      <div className="min-h-screen bg-dark-800 text-primary flex flex-col items-center justify-center p-4">
        <div className="card max-w-2xl w-full p-6">
          <h2 className="text-xl font-glitch mb-4 text-danger">
            ERROR::NFT_NOT_FOUND
          </h2>
          <p className="mb-6 font-mono text-primary/70">
            The requested asset #{tokenId} could not be located in your vault.
          </p>
          <button 
            onClick={() => navigate('/')}
            className="btn btn-primary font-mono"
          >
            RETURN_TO_VAULT
          </button>
        </div>
      </div>
    );
  }

  // Extract metadata
  const { metadata, imageUrl } = nft;

  return (
    <div className="min-h-screen bg-dark-800 text-primary p-4 md:p-8">
      <div className="max-w-6xl mx-auto">
        {/* Back button */}
        <button 
          onClick={() => navigate('/')}
          className="mb-6 flex items-center text-accent hover:text-accent/80 transition-colors font-mono"
        >
          <svg className="w-4 h-4 mr-2" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M19 12H5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            <path d="M12 19L5 12L12 5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
          </svg>
          RETURN_TO_VAULT
        </button>

        <div className="card overflow-hidden border border-primary/30">
          {/* Terminal header */}
          <div className="h-6 bg-dark-800 flex items-center">
            <div className="flex space-x-1 px-2">
              <div className="w-2 h-2 rounded-full bg-primary"></div>
              <div className="w-2 h-2 rounded-full bg-secondary"></div>
              <div className="w-2 h-2 rounded-full bg-accent"></div>
            </div>
            <div className="mx-auto font-mono text-xs text-primary/70">
              WAVS::NFT_ASSET::#{tokenId}
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 p-6">
            {/* Image section */}
            <div className="relative">
              {/* Tech frame for the image */}
              <div className="absolute -inset-px border border-primary/30"></div>
              
              {/* Scanlines effect */}
              <div className="absolute inset-0 z-10 pointer-events-none">
                <div
                  className="w-full h-full"
                  style={{
                    backgroundImage:
                      "repeating-linear-gradient(to bottom, transparent 0px, transparent 1px, rgba(0, 255, 65, 0.03) 1px, rgba(0, 255, 65, 0.03) 2px)",
                    backgroundSize: "100% 2px",
                  }}
                ></div>
              </div>
              
              {/* NFT Image */}
              {imageUrl ? (
                <div className="aspect-square overflow-hidden">
                  <img 
                    src={imageUrl} 
                    alt={metadata?.name || `NFT #${tokenId}`}
                    className="w-full h-full object-cover"
                  />
                </div>
              ) : (
                <div className="aspect-square bg-dark-900 flex items-center justify-center">
                  <div className="border-2 border-primary p-4">
                    <p className="text-primary/70 font-mono">NO_IMAGE_DATA</p>
                  </div>
                </div>
              )}

              {/* Token ID overlay */}
              <div className="absolute top-4 left-4 bg-dark-900/80 backdrop-blur-sm border border-primary/50 px-3 py-2">
                <div className="text-primary font-mono">#{tokenId}</div>
              </div>
            </div>

            {/* Metadata section */}
            <div className="space-y-6">
              {/* Title */}
              <div>
                <h1 className="text-2xl font-glitch mb-2 relative">
                  <span className="text-primary mr-2">[</span>
                  {metadata?.name || "UNNAMED_ASSET"}
                  <span className="text-primary ml-2">]</span>
                </h1>
                
                {/* Blockchain info */}
                <div className="flex items-center space-x-3 text-xs font-mono text-primary/60">
                  <div className="flex items-center">
                    <span className="w-2 h-2 bg-accent animate-pulse mr-2"></span>
                    <span>VERIFIED</span>
                  </div>
                  <div>IPFS::{nft.tokenURI.substring(0, 20)}...</div>
                  <a 
                    href={nft.tokenURI.replace("ipfs://", "https://gateway.lighthouse.storage/ipfs/")}
                    target="_blank"
                    rel="noopener noreferrer" 
                    className="text-accent hover:underline"
                  >
                    VIEW_METADATA
                  </a>
                </div>
              </div>
              
              {/* Description */}
              {metadata?.description && (
                <div className="space-y-2">
                  <div className="font-mono text-sm text-primary/80 border-b border-dark-700 pb-1">
                    DESCRIPTION
                  </div>
                  <div className="font-mono text-sm text-primary/90 leading-relaxed whitespace-pre-line">
                    {metadata.description}
                  </div>
                </div>
              )}
              
              {/* Attributes */}
              {metadata?.attributes && metadata.attributes.length > 0 && (
                <div className="space-y-3">
                  <div className="font-mono text-sm text-primary/80 border-b border-dark-700 pb-1">
                    ATTRIBUTES
                  </div>
                  <div className="grid grid-cols-2 gap-3">
                    {metadata.attributes.map((attr: any, i: number) => (
                      <div key={i} className="border border-dark-700 bg-dark-900/50 p-3">
                        <div className="text-xs text-primary/60 font-mono mb-1">
                          {attr.trait_type}
                        </div>
                        <div className="text-sm text-accent font-mono">
                          {attr.value}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
              
              {/* On-chain data */}
              <div className="space-y-3">
                <div className="font-mono text-sm text-primary/80 border-b border-dark-700 pb-1">
                  BLOCKCHAIN_DATA
                </div>
                <div className="space-y-2 font-mono text-sm">
                  <div className="flex justify-between">
                    <span className="text-primary/60">Token ID:</span>
                    <span className="text-primary">{tokenId}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-primary/60">Token Standard:</span>
                    <span className="text-primary">ERC-721</span>
                  </div>
                  <div className="flex">
                    <span className="text-primary/60 mr-2">Token URI:</span>
                    <span className="text-primary break-all text-xs">{nft.tokenURI}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default NFTDetailPage;