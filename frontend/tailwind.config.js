/** @type {import('tailwindcss').Config} */
const config = {
  darkMode: ["class"],
  content: ["./src/**/*.{html,js,svelte,ts}"],
  safelist: ["dark"],
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      colors: {
        border: "var(--border)",
        input: "var(--input)",
        ring: "var(--ring)",
        background: "var(--background)",
        foreground: "var(--foreground)",
        primary: {
          DEFAULT: "var(--clr-primary-100)",
          foreground: "var(--primary-foreground)",
          100: "var(--clr-primary-100)",
          200: "var(--clr-primary-200)",
          300: "var(--clr-primary-300)",
          400: "var(--clr-primary-400)",
          500: "var(--clr-primary-500)",
          600: "var(--clr-primary-600)",
          700: "var(--clr-primary-700)",
        },
        secondary: {
          DEFAULT: "var(--clr-secondary-300)",
          100: "var(--clr-secondary-100)",
          200: "var(--clr-secondary-200)",
          300: "var(--clr-secondary-300)",
          400: "var(--clr-secondary-400)",
          500: "var(--clr-secondary-500)",
          600: "var(--clr-secondary-600)",
          700: "var(--clr-secondary-700)",
        },
        destructive: {
          DEFAULT: "var(--destructive)",
          foreground: "var(--destructive-foreground)",
        },
        muted: {
          DEFAULT: "var(--muted)",
          foreground: "var(--muted-foreground)",
        },
        accent: {
          DEFAULT: "var(--accent)",
          foreground: "var(--accent-foreground)",
        },
        popover: {
          DEFAULT: "var(--popover)",
          foreground: "var(--popover-foreground)",
        },
        card: {
          DEFAULT: "var(--card)",
          foreground: "var(--card-foreground)",
        },
        success: "#0AB105",
        error: "#C50505",
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      fontFamily: {
        title: ["Montserrat", "sans-serif"],
      },
    },
  },
};

export default config;
