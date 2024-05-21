/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{html,svelte,js,ts,jsx,tsx}",
  ], 
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}

