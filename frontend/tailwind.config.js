/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{html,svelte,js,ts,jsx,tsx}",
  ], 
  theme: {
    extend: {
      colors: {
        primary: '#f9fafb',
        secondary: '#213547',
        secondary_light: '#1e293b',
        tertiary: '#d1d5db',
        invalid: '#db2777'
      }
    },
  },
  plugins: [
    require('@tailwindcss/forms')
  ],
}

