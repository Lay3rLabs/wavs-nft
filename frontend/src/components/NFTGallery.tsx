import React, { useState } from "react";
import { useMint } from "../contexts/MintContext";
import { useAccount } from "wagmi";

const NFTGallery: React.FC = () => {
  const { ownedNfts, pendingMints, loadingNfts, refreshNfts } = useMint();
  const { isConnected } = useAccount();
  const [selectedNft, setSelectedNft] = useState<string | null>(null);

  if (!isConnected) {
    return (
      <div className="card relative">
        <div className="absolute top-0 left-0 w-full h-6 bg-dark-800 flex items-center">
          <div className="flex space-x-1 px-2">
            <div className="w-2 h-2 rounded-full bg-danger"></div>
            <div className="w-2 h-2 rounded-full bg-warning"></div>
            <div className="w-2 h-2 rounded-full bg-primary"></div>
          </div>
          <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center">
            <span className="font-mono text-xs text-primary/70">
              TERMINAL::NFT_VAULT
            </span>
          </div>
        </div>

        <div className="mt-8 text-center py-12 border border-dashed border-primary/30">
          <div className="w-16 h-16 mx-auto border-2 border-primary flex items-center justify-center rounded-full mb-4">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-8 w-8 text-primary"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={1}
                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
              />
            </svg>
          </div>
          <h3 className="font-mono text-primary tracking-widest">
            ACCESS_REQUIRED
          </h3>
          <div className="console-output max-w-sm mx-auto mt-4 text-center">
            <div className="terminal-line">Authentication required</div>
            <div className="terminal-line">Connect wallet to proceed</div>
            <div className="text-danger mt-2">
              {">> STATUS: UNAUTHORIZED <<"}
            </div>
          </div>
        </div>
      </div>
    );
  }

  if (loadingNfts) {
    return (
      <div className="card">
        <div className="absolute top-0 left-0 w-full h-6 bg-dark-800 flex items-center">
          <div className="flex space-x-1 px-2">
            <div className="w-2 h-2 rounded-full bg-accent"></div>
            <div className="w-2 h-2 rounded-full bg-warning"></div>
            <div className="w-2 h-2 rounded-full bg-primary"></div>
          </div>
          <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center">
            <span className="font-mono text-xs text-primary/70">
              TERMINAL::LOADING
            </span>
          </div>
        </div>

        <div className="text-center py-16 mt-8">
          <div className="inline-block relative">
            <div className="absolute inset-0 bg-cyber-gradient opacity-20 animate-pulse rounded-full"></div>
            <div className="relative animate-spin rounded-full h-16 w-16 border-b-2 border-primary"></div>
            <div className="absolute inset-0 flex items-center justify-center">
              <span className="text-primary font-mono text-sm">LOAD</span>
            </div>
          </div>
          <div className="mt-6 font-mono text-primary/80 text-sm">
            <span className="animate-pulse">SCANNING BLOCKCHAIN...</span>
          </div>
          <div className="mt-2 font-mono text-xs text-primary/40">
            ETA: 3 SECONDS
          </div>
        </div>
      </div>
    );
  }

  const hasPendingOrOwned = pendingMints.length > 0 || ownedNfts.length > 0;

  // Simulating cyberpunk data metrics
  const securityLevel = Math.floor(Math.random() * 3) + 8; // 8, 9, or 10
  const networkLatency = Math.floor(Math.random() * 30) + 15; // Between 15-45ms
  const encryptedNfts = ownedNfts.length + pendingMints.length;

  return (
    <div className="card relative">
      {/* Terminal header */}
      <div className="absolute top-0 left-0 w-full h-6 bg-dark-800 flex items-center">
        <div className="flex space-x-1 px-2">
          <div className="w-2 h-2 rounded-full bg-primary"></div>
          <div className="w-2 h-2 rounded-full bg-secondary"></div>
          <div className="w-2 h-2 rounded-full bg-accent"></div>
        </div>
        <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center">
          <span className="font-mono text-xs text-primary/70">
            TERMINAL::NFT_VAULT
          </span>
        </div>
      </div>

      {/* Stats bar */}
      <div className="flex items-center justify-between mb-4 mt-8 font-mono text-xs border border-dark-700 bg-dark-900 p-2">
        <div className="text-primary">
          SEC::<span className="text-accent">{securityLevel}/10</span>
        </div>
        <div className="text-primary">
          LATENCY::<span className="text-warning">{networkLatency}ms</span>
        </div>
        <div className="text-primary">
          ENCRYPTED::<span className="text-secondary">{encryptedNfts}</span>
        </div>
        <div className="text-primary">
          IPFS::<span className="text-accent">ONLINE</span>
        </div>
      </div>

      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-glitch relative">
          <span className="text-primary mr-2">[</span>
          <span className="crt-flicker">ASSET_VAULT</span>
          <span className="text-primary ml-2">]</span>
          <span className="absolute -top-1 -right-3 w-2 h-2 bg-primary animate-pulse"></span>
        </h2>
        <button
          onClick={() => refreshNfts()}
          className="btn btn-secondary text-sm flex items-center"
        >
          <svg
            className="h-3 w-3 mr-1"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M2 10C2 10 4.00498 7.26822 5 6C8.5 2 13.5 2 17 6C18.0104 7.10851 19 10 19 10"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            <path
              d="M22 14C22 14 19.995 16.7318 19 18C15.5 22 10.5 22 7 18C5.98959 16.8915 5 14 5 14"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            <path
              d="M2 10H8V4"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
            <path
              d="M22 14H16V20"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
          RESCAN
        </button>
      </div>

      {!hasPendingOrOwned ? (
        <div className="border border-dashed border-dark-600 py-8 text-center relative">
          <div className="console-output max-w-md mx-auto">
            <div className="terminal-line">
              No assets found in current wallet
            </div>
            <div className="terminal-line">
              Run GENERATE_NFT to mint new asset
            </div>
            <div className="terminal-line text-primary/50">
              System ready for input...
            </div>
          </div>

          {/* Matrix-like binary decoration */}
          <div className="absolute bottom-2 right-2 font-mono text-xs text-primary/20">
            {Array.from({ length: 3 }).map((_, i) => (
              <div key={i} className="flex space-x-1">
                {Array.from({ length: 8 }).map((_, j) => (
                  <span key={j}>{Math.random() > 0.5 ? "1" : "0"}</span>
                ))}
              </div>
            ))}
          </div>
        </div>
      ) : (
        <div className="space-y-8">
          {pendingMints.length > 0 && (
            <div className="space-y-4">
              <h3 className="text-lg font-mono text-accent flex items-center">
                <span className="w-1 h-6 bg-accent mr-2"></span>
                PENDING_GENERATIONS
                <span className="ml-2 text-xs border border-accent px-1 bg-dark-900">
                  {pendingMints.length}
                </span>
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {pendingMints.map((mint) => (
                  <div
                    key={mint.triggerId}
                    className="relative border border-dark-600 p-4 bg-dark-800 overflow-hidden group"
                  >
                    {/* Status indicator */}
                    <div className="absolute top-0 right-0 border-l border-b border-dark-600 px-2 py-1 text-[10px] font-mono text-accent">
                      PROCESSING
                    </div>

                    {/* Digital noise background */}
                    <div className="h-40 bg-dark-900 relative overflow-hidden">
                      {/* Matrix lines effect */}
                      <div className="absolute inset-0 opacity-30">
                        {Array.from({ length: 8 }).map((_, i) => (
                          <div
                            key={i}
                            className="h-px bg-accent/30"
                            style={{
                              position: "absolute",
                              top: `${i * 12.5}%`,
                              left: 0,
                              right: 0,
                              transform:
                                i % 2 === 0 ? "scaleX(0.8)" : "scaleX(0.9)",
                              transformOrigin: i % 2 === 0 ? "left" : "right",
                            }}
                          />
                        ))}

                        {Array.from({ length: 12 }).map((_, i) => (
                          <div
                            key={i}
                            className="w-px h-full bg-accent/20"
                            style={{
                              position: "absolute",
                              left: `${i * 8.33}%`,
                              top: 0,
                              bottom: 0,
                            }}
                          />
                        ))}
                      </div>

                      {/* Loading indicator */}
                      <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2">
                        <div className="border-2 border-primary p-4 relative animate-pulse">
                          <div className="border border-primary p-2">
                            <div className="w-8 h-8 border border-primary bg-dark-900 flex items-center justify-center">
                              <div className="h-3 w-3 rounded-full bg-accent animate-pulse"></div>
                            </div>
                          </div>
                          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-16 h-px bg-cyber-gradient"></div>
                          <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-px h-16 bg-cyber-gradient"></div>
                        </div>
                      </div>
                    </div>

                    {/* Info section */}
                    <div className="mt-3 border-t border-dark-600 pt-3">
                      <div className="flex items-center font-mono">
                        <div className="w-2 h-2 bg-accent animate-pulse mr-2"></div>
                        <span className="text-xs text-accent">
                          ID::{mint.triggerId.substring(0, 8)}
                        </span>
                      </div>
                      <p className="text-xs text-primary/70 mt-2 font-mono leading-tight">
                        PROMPT::{mint.prompt.substring(0, 60)}
                        {mint.prompt.length > 60 ? "..." : ""}
                      </p>

                      {/* Loading progress */}
                      <div className="mt-2">
                        <div className="flex justify-between items-center">
                          <span className="text-xs text-primary/50 font-mono">
                            GENERATING
                          </span>
                          <span className="text-xs text-accent font-mono">
                            {Math.floor(Math.random() * 30) + 70}%
                          </span>
                        </div>
                        <div className="w-full h-1 bg-dark-900 mt-1">
                          <div
                            className="h-full bg-cyber-gradient animate-pulse"
                            style={{
                              width: `${Math.floor(Math.random() * 30) + 70}%`,
                            }}
                          ></div>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {ownedNfts.length > 0 && (
            <div className="space-y-4">
              <h3 className="text-lg font-mono text-primary flex items-center">
                <span className="w-1 h-6 bg-primary mr-2"></span>
                VAULT_ASSETS
                <span className="ml-2 text-xs border border-primary px-1 bg-dark-900">
                  {ownedNfts.length}
                </span>
              </h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {ownedNfts.map((nft) => (
                  <div
                    key={nft.tokenId}
                    className={`relative border group cursor-pointer transition-all bg-dark-800 overflow-hidden
                               ${
                                 selectedNft === nft.tokenId
                                   ? "border-cyber-gradient shadow-lg shadow-primary/20"
                                   : "border-dark-600 hover:border-primary/30"
                               }`}
                    onClick={() =>
                      setSelectedNft(
                        selectedNft === nft.tokenId ? null : nft.tokenId
                      )
                    }
                  >
                    {/* Status indicator */}
                    <div className="absolute top-0 right-0 border-l border-b border-dark-600 px-2 py-1 text-[10px] font-mono text-primary">
                      VERIFIED
                    </div>

                    {/* Token ID badge */}
                    <div className="absolute top-3 left-3 z-20 bg-dark-900/80 backdrop-blur-sm border border-primary/50 px-2 py-1">
                      <div className="text-primary font-mono text-sm">
                        #{nft.tokenId}
                      </div>
                    </div>

                    {/* Image frame */}
                    <div className="relative">
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

                      {nft.imageUrl ? (
                        <div className="relative h-48 overflow-hidden group-hover:opacity-90 transition-opacity">
                          <img
                            src={nft.imageUrl}
                            alt={`NFT #${nft.tokenId}`}
                            className="h-full w-full object-cover"
                          />
                          <div className="absolute inset-0 bg-gradient-to-t from-dark-900 to-transparent opacity-60"></div>
                        </div>
                      ) : (
                        <div className="h-48 bg-dark-900 flex items-center justify-center">
                          <div className="border-2 border-primary p-2">
                            <p className="text-xs text-primary/70 font-mono">
                              NO_IMAGE_DATA
                            </p>
                          </div>
                        </div>
                      )}
                    </div>

                    {/* NFT info */}
                    <div className="p-4">
                      <div className="flex items-center justify-between">
                        <div className="font-mono text-primary/90">
                          {nft.metadata?.name || "UNNAMED_ASSET"}
                        </div>
                        {/* Technical details button */}
                        <button
                          className="text-[10px] border border-primary/40 px-1 font-mono text-accent"
                          onClick={(e) => {
                            e.stopPropagation();
                            window.open(
                              nft.tokenURI.replace(
                                "ipfs://",
                                "https://ipfs.io/ipfs/"
                              ),
                              "_blank"
                            );
                          }}
                        >
                          IPFS_DATA
                        </button>
                      </div>

                      {/* Metadata preview */}
                      {selectedNft === nft.tokenId && nft.metadata && (
                        <div className="mt-3 border-t border-dark-700 pt-3 font-mono text-xs">
                          <div className="console-output text-[10px] max-h-24 overflow-y-auto">
                            {nft.metadata.description ? (
                              <>
                                <div className="text-primary/60"> DESC::</div>
                                <div className="pl-2 text-primary/90 mb-2">
                                  {nft.metadata.description}
                                </div>
                              </>
                            ) : null}

                            {nft.metadata.attributes ? (
                              <>
                                <div className="text-primary/60">
                                  {" "}
                                  ATTRIBUTES::
                                </div>
                                <div className="pl-2 text-primary/90">
                                  {nft.metadata.attributes.map(
                                    (attr: any, i: number) => (
                                      <div key={i}>
                                        {attr.trait_type}:{" "}
                                        <span className="text-accent">
                                          {attr.value}
                                        </span>
                                      </div>
                                    )
                                  )}
                                </div>
                              </>
                            ) : null}
                          </div>
                        </div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default NFTGallery;
