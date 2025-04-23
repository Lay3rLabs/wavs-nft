/// <reference types="vite/client" />

interface Window {
  ethereum: any;
}

declare module '*.svg' {
  const content: any;
  export default content;
}

declare module '*.json' {
  const content: any;
  export default content;
}