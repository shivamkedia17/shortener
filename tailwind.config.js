/** @type {import('tailwindcss').Config} */

const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  content: ["./src/**/*.{html,js}", "./site/**/*.{html,hbs}"],
  theme: {
    extend: {},
  },
  plugins: [],
};
