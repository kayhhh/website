/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      animation: {
        "float": "float 3s ease-in-out infinite",
        "float-2": "float2 3s ease-in-out infinite",
        "marquee": "marquee 20s alternate linear infinite",
        "marquee-title": "marquee-title 80s linear infinite",
      },
      keyframes: {
        "float": {
          "0%": { transform: "translateY(0%)" },
          "50%": { transform: "translateY(6%)" },
          "100%": { transform: "translateY(0%)" },
        },
        float2: {
          "0%": { transform: "translateY(6%)" },
          "50%": { transform: "translateY(0%)" },
          "100%": { transform: "translateY(6%)" },
        },
        "marquee": {
          // Goldren Ratio Spiral
          "0%": { transform: "scale(100%) translateX(0%) translateY(0%)" },
          "5%": { transform: "scale(95%) translateX(1%) translateY(1%)" },
          "10%": { transform: "scale(90%) translateX(2%) translateY(1%)" },
          "15%": { transform: "scale(85%) translateX(0%) translateY(-3%)" },
          "20%": { transform: "scale(80%) translateX(-3%) translateY(1%)" },
          "25%": { transform: "scale(75%) translateX(2%) translateY(5%)" },
          "30%": { transform: "scale(70%) translateX(10%) translateY(-3%)" },
          "35%": { transform: "scale(65%) translateX(-3%) translateY(-15%)" },
          "40%": { transform: "scale(60%) translateX(-24%) translateY(6%)" },
          "45%": { transform: "scale(55%) translateX(10%) translateY(40%)" },
          "50%": { transform: "scale(50%) translateX(65%) translateY(-15%)" },
          "55%": { transform: "scale(45%) translateX(-24%) translateY(-60%)" },
          "60%": { transform: "scale(40%) translateX(-89%) translateY(25%)" },
          "65%": { transform: "scale(35%) translateX(65%) translateY(100%)" },
          "70%": { transform: "scale(30%) translateX(233%) translateY(-60%)" },
          "75%": { transform: "scale(25%) translateX(-89%) translateY(-225%)" },
          "80%": { transform: "scale(20%) translateX(-322%) translateY(100%)" },
          "85%": { transform: "scale(15%) translateX(233%) translateY(400%)" },
          "90%": { transform: "scale(10%) translateX(987%) translateY(-225%)" },
          "95%": { transform: "scale(5%) translateX(-322%) translateY(-985%)" },
          "100%": { transform: "scale(0%) translateX(-1311%) translateY(400%)" },
        },
        "marquee-title": {
          "0%": { transform: "translateX(100%)" },
          "100%": { transform: "translateX(-900%)" },
        },
      },
    },
  },
  plugins: [],
};
