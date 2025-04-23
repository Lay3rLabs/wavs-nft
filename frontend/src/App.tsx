import React, { useEffect, useState } from "react";
import { Routes, Route } from "react-router-dom";
import Header from "./components/Header";
import MintForm from "./components/MintForm";
import NFTGallery from "./components/NFTGallery";
import NFTDetailPage from "./components/NFTDetailPage";
import HomePage from "./components/HomePage";
import DebugInfo from "./components/DebugInfo";
import { MintProvider } from "./contexts/MintContext";
import { addLocalNetwork } from "./utils/addLocalNetwork";

const MatrixRain: React.FC = () => {
  const [columns, setColumns] = useState<number[]>([]);

  useEffect(() => {
    // Generate random columns for matrix effect
    const numColumns = Math.floor(window.innerWidth / 20); // approx. character width
    const cols = Array.from({ length: numColumns }, () => 0);
    setColumns(cols);

    const interval = setInterval(() => {
      setColumns((prevCols) => {
        return prevCols.map((y) => {
          // Random chance to reset column
          if (Math.random() > 0.98) {
            return 0;
          }
          // Otherwise increment with wrapping
          return (y + 1) % (window.innerHeight / 20);
        });
      });
    }, 100);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="fixed inset-0 pointer-events-none opacity-10 z-10 overflow-hidden">
      <div className="absolute inset-0 text-primary font-mono">
        {columns.map((y, i) => (
          <div
            key={i}
            className="absolute whitespace-pre"
            style={{ left: `${i * 20}px`, top: `${y * 20}px` }}
          >
            {Math.random() > 0.5 ? "1" : "0"}
          </div>
        ))}
      </div>
    </div>
  );
};

const BOOT_MESSAGES = [
  "INITIALIZING SYSTEM...",
  "LOADING KERNEL...",
  "ACCESSING BLOCKCHAIN...",
  "ESTABLISHING IPFS CONNECTION...",
  "CONFIGURING ENCRYPTION...",
  "INITIALIZING WAVS PROTOCOL...",
  "VERIFYING CRYPTOGRAPHIC KEYS...",
  "SYSTEM READY",
];

const App: React.FC = () => {
  const [bootSequence, setBootSequence] = useState(true);
  const [bootPhase, setBootPhase] = useState(0);
  const [loadingText, setLoadingText] = useState("");

  // Add the local Anvil network to MetaMask when the app loads
  useEffect(() => {
    const setupLocalNetwork = async () => {
      try {
        // Check if we need to add the network
        if (window.ethereum && process.env.NODE_ENV !== 'production') {
          // Listen for chain changes to detect network issues
          const handleChainChanged = (chainId: string) => {
            console.log('Chain changed to:', chainId);
            // If chainId is not our expected local chain (1337), prompt to add it
            if (chainId !== '0x539') { // 1337 in hex
              console.log('Not on local network, attempting to add and switch...');
              addLocalNetwork().catch(console.error);
            }
          };

          // Add event listener
          window.ethereum.on('chainChanged', handleChainChanged);
          
          // Initial check and setup
          const chainId = await window.ethereum.request({ method: 'eth_chainId' });
          handleChainChanged(chainId);
          
          // Cleanup
          return () => {
            if (window.ethereum && window.ethereum.removeListener) {
              window.ethereum.removeListener('chainChanged', handleChainChanged);
            }
          };
        }
      } catch (error) {
        console.error('Error setting up local network:', error);
      }
    };
    
    setupLocalNetwork();
  }, []);

  // Boot sequence effect
  useEffect(() => {
    if (!bootSequence) return;

    const timer = setTimeout(
      () => {
        if (bootPhase < BOOT_MESSAGES.length) {
          setLoadingText(BOOT_MESSAGES[bootPhase]);
          setBootPhase((prev) => prev + 1);
        } else {
          setBootSequence(false);
        }
      },
      bootPhase === 0 ? 500 : 300
    );

    return () => clearTimeout(timer);
  }, [bootSequence, bootPhase]);

  if (bootSequence) {
    return (
      <div className="min-h-screen bg-black flex items-center justify-center font-mono text-primary relative overflow-hidden">
        <div
          className="fixed inset-0"
          style={{
            backgroundImage:
              "repeating-linear-gradient(transparent 0px, rgba(0, 255, 65, 0.03) 1px, transparent 2px)",
            backgroundSize: "100% 3px",
          }}
        ></div>

        <div className="max-w-2xl w-full px-4">
          <div className="border-2 border-primary p-8 relative">
            <div className="absolute top-0 left-0 w-full h-6 bg-dark-800 -mt-6 flex items-center">
              <div className="flex space-x-1 px-2">
                <div className="w-2 h-2 rounded-full bg-primary"></div>
                <div className="w-2 h-2 rounded-full bg-secondary"></div>
                <div className="w-2 h-2 rounded-full bg-accent"></div>
              </div>
              <div className="absolute top-0 left-0 w-full h-full flex items-center justify-center">
                <span className="font-mono text-xs text-primary/70">
                  WAVS::BOOT_SEQUENCE
                </span>
              </div>
            </div>

            <div className="mb-6 font-glitch text-xl flex items-center">
              <div className="w-3 h-3 bg-primary mr-3 animate-pulse"></div>
              WAVS AUTONOMOUS SYSTEM
            </div>

            <div className="mb-6 space-y-2 text-sm">
              {Array.from({ length: bootPhase }).map((_, i) => (
                <div
                  key={i}
                  className={`flex items-start ${
                    i === bootPhase - 1 ? "text-accent" : ""
                  }`}
                >
                  <span className="mr-2">
                    [<span className="text-accent">SYS</span>]
                  </span>
                  <span>{BOOT_MESSAGES[i]}</span>
                  {i === bootPhase - 1 && i !== BOOT_MESSAGES.length - 1 && (
                    <span className="animate-pulse ml-1">...</span>
                  )}
                  {i === BOOT_MESSAGES.length - 1 && (
                    <span className="text-primary">✓</span>
                  )}
                </div>
              ))}
            </div>

            {bootPhase < BOOT_MESSAGES.length && (
              <div className="h-1 bg-dark-800 w-full">
                <div
                  className="h-full bg-cyber-gradient transition-all duration-300"
                  style={{
                    width: `${(bootPhase / BOOT_MESSAGES.length) * 100}%`,
                  }}
                ></div>
              </div>
            )}

            <div className="mt-4 text-xs text-primary/50">
              {bootPhase >= BOOT_MESSAGES.length ? (
                <div className="text-center animate-pulse">
                  PRESS ANY KEY TO CONTINUE
                </div>
              ) : (
                <div className="flex justify-between">
                  <span>
                    BOOT SEQUENCE:{" "}
                    {Math.round((bootPhase / BOOT_MESSAGES.length) * 100)}%
                  </span>
                  <span>{new Date().toISOString()}</span>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Auto-continue after boot */}
        {bootPhase >= BOOT_MESSAGES.length && (
          <div
            className="fixed inset-0 cursor-pointer"
            onClick={() => setBootSequence(false)}
            onKeyDown={() => setBootSequence(false)}
            tabIndex={0}
          />
        )}
      </div>
    );
  }

  return (
    <MintProvider>
      <div className="min-h-screen flex flex-col relative">
        {/* Matrix-like animation background */}
        <MatrixRain />

        {/* Scan line effect */}
        <div className="fixed inset-0 pointer-events-none z-20">
          <div className="absolute top-0 left-0 w-full h-screen overflow-hidden opacity-5">
            <div
              className="w-full h-screen"
              style={{
                backgroundImage:
                  "repeating-linear-gradient(0deg, transparent, transparent 1px, rgba(0, 255, 65, 0.1) 1px, rgba(0, 255, 65, 0.1) 2px)",
                backgroundSize: "100% 2px",
              }}
            ></div>
          </div>
        </div>
        
        {/* Debug Panel (toggle with Ctrl+Shift+D) */}
        <DebugInfo />

        <Routes>
          <Route path="/" element={<HomePage />} />
          <Route path="/nft/:tokenId" element={<NFTDetailPage />} />
        </Routes>
      </div>
    </MintProvider>
  );
};

export default App;
