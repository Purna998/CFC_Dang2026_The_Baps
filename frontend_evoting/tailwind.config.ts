import type { Config } from "tailwindcss";

const config: Config = {
  darkMode: ["class"],
  content: [
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        "deep-navy": "#06152F",
        "primary-dark": "#00081E",
        "primary-container": "#0a1f44",
        "soft-blue-bg": "#EEF3F9",
        "border-gray": "#DDE4EC",
        "secondary-crimson": "#dc2f42",
        "dark-red": "#9F172B",
        "emerald-green": "#2C9969",
        "warning-gold": "#D99A21",
        "surface-bright": "#f7f9fc",
        "surface-container": "#eceef1",
        "surface-variant": "#e0e3e6",
        "on-surface-variant": "#44464e",
      },
      fontFamily: {
        sans: ["var(--font-inter)", "sans-serif"],
        display: ["var(--font-plus-jakarta)", "sans-serif"],
        mono: ["var(--font-jetbrains-mono)", "monospace"],
      },
      animation: {
        'pulse-slow': 'pulse 4s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px rgba(220, 47, 66, 0.2), 0 0 20px rgba(6, 21, 47, 0.4)' },
          '100%': { boxShadow: '0 0 20px rgba(220, 47, 66, 0.6), 0 0 35px rgba(44, 153, 105, 0.6)' },
        }
      }
    },
  },
  plugins: [],
};
export default config;
