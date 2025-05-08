import React, { useState, useEffect } from 'react';
import { useMint } from '../contexts/MintContext';
import { useAccount } from 'wagmi';
import toast from 'react-hot-toast';

const MintForm: React.FC = () => {
  const [prompt, setPrompt] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [cursorChar, setCursorChar] = useState('_');
  const { mintPrice, triggerMint } = useMint();
  const { isConnected } = useAccount();

  // Blinking cursor effect
  useEffect(() => {
    const interval = setInterval(() => {
      setCursorChar(prev => prev === '_' ? ' ' : '_');
    }, 530);
    return () => clearInterval(interval);
  }, []);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!prompt.trim()) {
      toast.error('ERROR::PROMPT_MISSING', {
        style: {
          border: '1px solid #FF003C',
          padding: '16px',
          color: '#FF003C',
          background: '#121212'
        },
        iconTheme: {
          primary: '#FF003C',
          secondary: '#121212',
        },
      });
      return;
    }

    if (!isConnected) {
      toast.error('ERROR::WALLET_DISCONNECTED', {
        style: {
          border: '1px solid #FF003C',
          padding: '16px',
          color: '#FF003C',
          background: '#121212'
        },
        iconTheme: {
          primary: '#FF003C',
          secondary: '#121212',
        },
      });
      return;
    }

    try {
      setIsSubmitting(true);
      const toastId = toast.loading('INITIALIZING_TRANSFER', {
        style: {
          border: '1px solid #00FFFF',
          padding: '16px',
          color: '#00FFFF',
          background: '#121212'
        },
      });
      
      const triggerId = await triggerMint(prompt);
      
      if (triggerId) {
        toast.success(`MINT::SUCCESS::ID=${triggerId}`, { 
          id: toastId,
          style: {
            border: '1px solid #00FF41',
            padding: '16px',
            color: '#00FF41',
            background: '#121212'
          },
          iconTheme: {
            primary: '#00FF41',
            secondary: '#121212',
          },
        });
        toast.loading(
          'GENERATING_NFT::ESTIMATED_TIME=180s',
          { 
            duration: 5000,
            style: {
              border: '1px solid #00FFFF',
              padding: '16px',
              color: '#00FFFF',
              background: '#121212'
            },
          }
        );
        setPrompt('');
      } else {
        toast.error('ERROR::MINT_FAILED::CODE=500', { 
          id: toastId,
          style: {
            border: '1px solid #FF003C',
            padding: '16px',
            color: '#FF003C',
            background: '#121212'
          },
          iconTheme: {
            primary: '#FF003C',
            secondary: '#121212',
          },
        });
      }
    } catch (error) {
      console.error('Mint error:', error);
      toast.error(`ERROR::EXCEPTION::${error instanceof Error ? error.message : 'UNKNOWN'}`, {
        style: {
          border: '1px solid #FF003C',
          padding: '16px',
          color: '#FF003C',
          background: '#121212'
        },
        iconTheme: {
          primary: '#FF003C',
          secondary: '#121212',
        },
      });
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="card mb-8">
      {/* Status indicators */}
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-glitch tracking-wider relative">
          <span className="text-primary mr-2">[</span>
          INITIALIZE_NEW_ASSET
          <span className="text-primary ml-2">]</span>
          <span className="absolute -top-1 -right-3 w-2 h-2 bg-primary animate-pulse"></span>
        </h2>
        <div className="flex items-center space-x-3">
          <div className="text-xs font-mono">
            <span className="text-primary">STATUS:</span>
            <span className={`ml-1 ${isSubmitting ? 'text-secondary animate-pulse' : 'text-accent'}`}>
              {isSubmitting ? 'PROCESSING' : 'READY'}
            </span>
          </div>
          <div className="h-4 w-px bg-dark-600"></div>
          <div className="text-xs font-mono">
            <span className="text-primary">SYS:</span>
            <span className="ml-1 text-warning">ONLINE</span>
          </div>
        </div>
      </div>

      {/* Matrix-like background decoration */}
      <div className="absolute -inset-[1px] overflow-hidden opacity-20 -z-10 pointer-events-none">
        <div className="absolute inset-0 overflow-hidden">
          {Array.from({ length: 5 }).map((_, i) => (
            <div 
              key={i}
              className="absolute text-primary font-mono text-xs"
              style={{
                left: `${Math.random() * 100}%`,
                top: `${Math.random() * 100}%`,
                opacity: 0.7
              }}
            >
              {Array.from({ length: 3 }).map((_, j) => (
                <div key={j}>{Math.random() > 0.5 ? '1' : '0'}</div>
              ))}
            </div>
          ))}
        </div>
      </div>
      
      <form onSubmit={handleSubmit} className="space-y-6">
        <div>
          <div className="flex items-center justify-between mb-2">
            <label htmlFor="prompt" className="label flex items-center">
              <span className="w-2 h-2 bg-primary mr-2"></span>
              INPUT_PROMPT
            </label>
            <div className="text-xs font-mono text-accent">BUFFER: {prompt.length} CHARS</div>
          </div>
          
          <div className="relative">
            {/* Terminal-style textbox with custom border */}
            <div className="absolute -inset-[1px] bg-cyber-gradient opacity-40 blur-[1px]"></div>
            <textarea
              id="prompt"
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              placeholder={`Describe your vision for generation${cursorChar}`}
              className="input w-full h-32 resize-none font-mono bg-dark-900 z-10 relative 
                         border-0 focus:ring-0 focus:shadow-none focus:border-none"
              disabled={isSubmitting}
              required
              style={{
                backgroundImage: `
                  linear-gradient(transparent, transparent 19px, rgba(0, 255, 65, 0.05) 19px, rgba(0, 255, 65, 0.05) 20px)
                `,
                backgroundSize: '100% 20px',
                lineHeight: '20px'
              }}
            />
            
            {/* Security pattern decoration */}
            <div className="absolute bottom-3 right-4 text-primary/40 font-mono text-xs">
              <span className="tracking-widest">::SECURE::</span>
            </div>
          </div>
          
          {/* Techie details */}
          <div className="mt-2 flex items-center justify-between text-xs text-primary/60 font-mono">
            <div>ENCRYPTION: AES-256</div>
            <div>IPFS: ENABLED</div>
            <div>PERSISTENCE: PERMANENT</div>
          </div>
        </div>
        
        <div className="border-t border-dark-700 pt-4 pb-2 flex items-center justify-between">
          <div className="flex flex-col">
            <div className="text-xs font-mono text-warning">COST_ANALYSIS:</div>
            <div className="font-mono text-accent text-lg">
              <span className="font-glitch">{mintPrice}</span>
              <span className="ml-1 text-xs text-primary/80">ETH</span>
            </div>
            <div className="text-xs text-primary/60 font-mono mt-1">NETWORK_FEE: VARIABLE</div>
          </div>
          
          <div className="relative">
            {/* Neon button effect */}
            {!isSubmitting && isConnected && (
              <div className="absolute -inset-[2px] bg-cyber-gradient opacity-50 blur-[3px] rounded-sm"></div>
            )}
            <button
              type="submit"
              className={`relative btn btn-primary px-8 py-3 font-mono tracking-widest z-10
                          disabled:opacity-50 disabled:pointer-events-none disabled:shadow-none 
                          disabled:border-dark-600 disabled:text-primary/50`}
              disabled={isSubmitting || !isConnected}
            >
              {isSubmitting ? (
                <div className="flex items-center justify-center">
                  <div className="h-4 w-4 border-2 border-t-transparent border-primary animate-spin mr-2 rounded-full"></div>
                  <span className="animate-pulse">PROCESSING...</span>
                </div>
              ) : (
                <>GENERATE_NFT</>
              )}
            </button>
          </div>
        </div>
      </form>
    </div>
  );
};

export default MintForm;