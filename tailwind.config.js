/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "class",
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
    "node_modules/flowbite/**/*.js",
  ],
  theme: {
    extend: {},
    variants: {
      extend: {
        backdropBlur: ["dark"],
        ringColor: ["dark"],
        ringWidth: ["dark"],
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
