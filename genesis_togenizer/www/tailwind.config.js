/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'cyberpunk-blue': '#06b6d4',
        'cyberpunk-purple': '#a855f7',
        'cyberpunk-pink': '#ec4899',
        'cyberpunk-dark': '#0f0f0f',
      },
      fontFamily: {
        mono: ['"JetBrains Mono"', '"Fira Code"', 'monospace'],
      },
    },
  },
  plugins: [],
}