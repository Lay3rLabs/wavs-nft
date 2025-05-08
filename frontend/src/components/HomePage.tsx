import React from 'react';
import Header from './Header';
import MintForm from './MintForm';
import NFTGallery from './NFTGallery';

const HomePage: React.FC = () => {
  return (
    <>
      <Header />
      
      <main className="flex-1 py-8 px-4 md:px-8 max-w-6xl mx-auto w-full">
        <section className="mb-12 relative">
          {/* Decorative circuit pattern */}
          <div className="absolute -top-4 -left-4 w-32 h-32 opacity-20 pointer-events-none">
            <div className="absolute top-0 left-0 w-full h-px bg-primary"></div>
            <div className="absolute top-0 left-0 h-full w-px bg-primary"></div>
            <div className="absolute top-[25%] left-0 w-1/2 h-px bg-primary"></div>
            <div className="absolute top-0 left-[25%] h-1/2 w-px bg-primary"></div>
            <div className="absolute top-[25%] left-[25%] w-2 h-2 bg-primary"></div>
            <div className="absolute top-[50%] left-0 w-3/4 h-px bg-primary"></div>
            <div className="absolute top-0 left-[50%] h-3/4 w-px bg-primary"></div>
            <div className="absolute top-[75%] left-0 w-1/2 h-px bg-primary"></div>
            <div className="absolute top-0 left-[75%] h-1/2 w-px bg-primary"></div>
          </div>

          <div className="text-center mb-12 relative">
            <div className="inline-block relative">
              <h1 className="text-4xl font-glitch mb-4 relative z-10 tracking-wider">
                <span className="crt-flicker text-primary">CRYPTO</span>
                <span className="text-accent">WAVE</span>
                <span className="text-secondary">NFT</span>
              </h1>
              <div className="absolute -bottom-2 left-0 w-full h-px bg-cyber-gradient"></div>
            </div>

            <p className="text-primary/70 max-w-2xl mx-auto font-mono text-sm">
              GENERATE SECURE || AUTONOMOUS || DECENTRALIZED ASSETS
              <br />
              <span className="text-xs text-primary/50">
                ALL TRANSACTIONS ENCRYPTED • IMMUTABLE STORAGE • HIGH ENTROPY
                RANDOMIZATION
              </span>
            </p>

            {/* Technical decoration */}
            <div className="absolute -right-4 top-0 opacity-30 text-[8px] font-mono text-primary">
              SHA256::e6fb06210fafc02fd7479ddbed2d042cc3a5155e4411d7b1b7787f6e05e1a
            </div>
          </div>

          <div className="max-w-2xl mx-auto">
            <MintForm />
          </div>
        </section>

        <section>
          <NFTGallery />
        </section>
      </main>

      <footer className="py-6 text-center font-mono text-primary/40 text-xs border-t border-dark-700 relative">
        <div className="max-w-6xl mx-auto px-4 flex flex-col md:flex-row justify-between items-center">
          <div className="mb-2 md:mb-0">
            <span className="text-primary/80">WAVS:AUTONOMOUS:SYSTEM</span> ©{" "}
            {new Date().getFullYear()}
          </div>

          <div className="flex space-x-4">
            <div>BLOCK::{Math.floor(Math.random() * 1000000)}</div>
            <div>
              HASH::0x
              {Array.from({ length: 8 }, () =>
                Math.floor(Math.random() * 16).toString(16)
              ).join("")}
            </div>
            <div className="text-accent">V1.0.2</div>
          </div>
        </div>

        {/* Background pattern */}
        <div className="absolute bottom-0 left-0 w-full h-px bg-cyber-gradient opacity-30"></div>
      </footer>
    </>
  );
};

export default HomePage;