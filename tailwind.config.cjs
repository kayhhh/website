/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      animation: {
        marquee: "marquee 80s linear infinite",
      },
      keyframes: {
        marquee: {
          "0%": { transform: "translateX(100%)" },
          "100%": { transform: "translateX(-800%)" },
        },
      },
    },
  },
  plugins: [],
};
