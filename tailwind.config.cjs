/** @type {import('tailwindcss').Config}*/
const config = {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    fontFamily: {
      'sigmar': ["Sigmar", "sans-serif"],
    },
    extend: {},
  },

  plugins: [],
};

module.exports = config;
