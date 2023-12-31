/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      animation: {
        float: "float 3s ease-in-out infinite",
        float2: "float2 3s ease-in-out infinite",
      },
      keyframes: {
        float: {
          "0%": { transform: "translateY(0%)" },
          "50%": { transform: "translateY(6%)" },
          "100%": { transform: "translateY(0%)" },
        },
        float2: {
          "0%": { transform: "translateY(6%)" },
          "50%": { transform: "translateY(0%)" },
          "100%": { transform: "translateY(6%)" },
        },
      },
    },
  },
  plugins: [],
};
