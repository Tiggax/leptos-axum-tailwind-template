/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      colors: {
        bg: {
          1: "hsl(var(--color-bg) / <alpha-value>)",
          2: "hsl(var(--color-bg2) / <alpha-value>)",
        },
        content: {
          1: "hsl(var(--color-content) / <alpha-value>)",
          2: "hsl(var(--color-content2) / <alpha-value>)",
        },
        accent: {
          1: "hsl(var(--color-accent) / <alpha-value>)",
          2: "hsl(var(--color-accent2) / <alpha-value>)",
        },
        dbg: {
          succ: "hsl(var(--color-succsess) / <alpha-value>)",
          info: "hsl(var(--color-info) / <alpha-value>)",
          warn: "hsl(var(--color-warning) / <alpha-value>)",
          dang: "hsl(var(--color-danger) / <alpha-value>)",
        },
      },
    },
  },
  plugins: [],
}
