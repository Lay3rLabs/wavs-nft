import React from 'react';
import { ConnectButton } from '@rainbow-me/rainbowkit';

const Header: React.FC = () => {
  return (
    <header className="w-full py-6 px-4 md:px-8 flex justify-between items-center relative border-b border-dark-700 backdrop-blur-sm z-10">
      {/* Animated header underline */}
      <div className="absolute bottom-0 left-0 w-full h-[1px] bg-cyber-gradient"></div>
      
      {/* Network/blockchain decor */}
      <div className="absolute top-1/2 left-0 w-full h-px bg-dark-600 -z-10"></div>
      <div className="absolute top-0 left-1/4 bottom-0 w-px bg-dark-600 -z-10"></div>
      <div className="absolute top-0 left-3/4 bottom-0 w-px bg-dark-600 -z-10"></div>
      
      <div className="flex items-center">
        <div className="h-8 w-8 mr-3 relative">
          <div className="absolute inset-0 border-2 border-primary animate-pulse"></div>
          <div className="absolute inset-[3px] border border-primary/60"></div>
          <div className="absolute inset-0 flex items-center justify-center text-primary font-mono text-xs">W</div>
        </div>
        <h1 className="text-xl md:text-2xl font-bold font-glitch text-white relative group">
          <span className="crt-flicker">W</span>
          <span className="">A</span>
          <span className="crt-flicker">V</span>
          <span className="">S::</span>
          <span className="text-primary ml-2">NFT</span>
          <span className="bg-dark-800 text-secondary px-2 ml-2 text-sm">v1.0</span>
          
          {/* Top secret hover effect */}
          <span className="absolute -top-1 -right-1 bg-danger text-black text-[8px] px-1 rotate-12 
                         opacity-0 group-hover:opacity-100 transition-opacity">
            TOP SECRET
          </span>
        </h1>
      </div>
      
      {/* Status indicators */}
      <div className="hidden md:flex items-center space-x-4 mx-4">
        <div className="flex items-center">
          <div className="h-2 w-2 rounded-full bg-primary animate-pulse mr-2"></div>
          <span className="text-xs text-primary font-mono">NODE:ACTIVE</span>
        </div>
        <div className="flex items-center">
          <div className="h-2 w-2 rounded-full bg-warning mr-2"></div>
          <span className="text-xs text-warning font-mono">LINK:SECURE</span>
        </div>
      </div>
      
      {/* Custom ConnectButton wrapper */}
      <div className="relative">
        <div className="absolute -inset-[1px] bg-cyber-gradient opacity-50 blur-[1px] -z-10"></div>
        <ConnectButton.Custom>
          {({
            account,
            chain,
            openAccountModal,
            openChainModal,
            openConnectModal,
            mounted,
          }) => {
            const ready = mounted;
            const connected = ready && account && chain;

            return (
              <div
                {...(!ready && {
                  'aria-hidden': true,
                  style: {
                    opacity: 0,
                    pointerEvents: 'none',
                    userSelect: 'none',
                  },
                })}
              >
                {(() => {
                  if (!connected) {
                    return (
                      <button 
                        onClick={openConnectModal} 
                        className="bg-dark-800 text-primary border border-primary px-4 py-2 
                                  font-mono text-sm uppercase tracking-wider
                                  hover:bg-primary/10 hover:shadow-[0_0_8px_0] hover:shadow-primary/50
                                  transition-all duration-300"
                      >
                        <div className="flex items-center">
                          <span className="mr-2">CONNECT</span>
                          <svg className="h-3 w-3 text-primary" fill="currentColor" viewBox="0 0 16 16">
                            <path d="M12.982 11.753a.5.5 0 00-.5-.5h-7.5a.5.5 0 00-.5.5v1.5a.5.5 0 00.5.5h7.5a.5.5 0 00.5-.5v-1.5zM4.982 6.753a.5.5 0 01.5-.5h7.5a.5.5 0 01.5.5v1.5a.5.5 0 01-.5.5h-7.5a.5.5 0 01-.5-.5v-1.5zM12.982 1.753a.5.5 0 00-.5-.5h-7.5a.5.5 0 00-.5.5v1.5a.5.5 0 00.5.5h7.5a.5.5 0 00.5-.5v-1.5z" />
                          </svg>
                        </div>
                      </button>
                    );
                  }

                  return (
                    <div className="flex items-center">
                      <button
                        onClick={openChainModal}
                        className="flex items-center bg-dark-800 text-sm border border-r-0 border-accent 
                                  px-3 py-2 font-mono text-accent hover:bg-accent/10 transition"
                      >
                        {chain.iconUrl && (
                          <img
                            alt={chain.name ?? 'Chain icon'}
                            src={chain.iconUrl}
                            className="h-4 w-4 mr-2"
                          />
                        )}
                        {chain.name.toUpperCase()}
                      </button>

                      <button
                        onClick={openAccountModal}
                        className="flex items-center bg-dark-800 text-sm border border-primary 
                                  px-3 py-2 font-mono text-primary hover:bg-primary/10 transition"
                      >
                        <span className="inline-block w-2 h-2 rounded-full bg-primary animate-pulse mr-2"></span>
                        {account.displayName}
                      </button>
                    </div>
                  );
                })()}
              </div>
            );
          }}
        </ConnectButton.Custom>
      </div>
    </header>
  );
};

export default Header;