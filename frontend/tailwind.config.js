/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: '#00FF41', // Matrix green
        secondary: '#FF00FF', // Cyberpunk magenta
        accent: '#00FFFF', // Cyber cyan
        danger: '#FF003C', // Neon red
        warning: '#FFFC00', // Neon yellow
        background: '#000000',
        foreground: '#E2E8F0',
        dark: {
          900: '#0A0A0A',
          800: '#121212',
          700: '#1A1A1A',
          600: '#222222',
        },
      },
      fontFamily: {
        mono: ['Courier Prime', 'monospace'],
        glitch: ['Orbitron', 'sans-serif'],
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glitch': 'glitch 1s linear infinite',
        'flicker': 'flicker 3s linear infinite',
        'scan-line': 'scan-line 8s linear infinite',
      },
      keyframes: {
        glitch: {
          '0%, 2%, 5%, 8%, 100%': { transform: 'none' },
          '3%': { transform: 'skewX(30deg)' },
          '6%': { transform: 'skewX(-30deg)' },
        },
        flicker: {
          '0%': { opacity: '1' },
          '10%': { opacity: '0.95' },
          '20%': { opacity: '1' },
          '35%': { opacity: '0.85' },
          '40%': { opacity: '0.9' },
          '60%': { opacity: '1' },
          '65%': { opacity: '0.85' },
          '80%': { opacity: '1' },
          '95%': { opacity: '0.9' },
          '100%': { opacity: '1' },
        },
        'scan-line': {
          '0%': { top: '0%' },
          '100%': { top: '100%' },
        },
      },
      backgroundImage: {
        'grid-pattern': 'linear-gradient(to right, #0A0A0A 1px, transparent 1px), linear-gradient(to bottom, #0A0A0A 1px, transparent 1px)',
        'cyber-gradient': 'linear-gradient(90deg, #00FFFF 0%, #FF00FF 100%)',
        'matrix-gradient': 'linear-gradient(180deg, #000000 0%, #003300 100%)',
      },
    },
  },
  plugins: [],
}