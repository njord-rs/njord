/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    container: {
      center: true,
      padding: {
        DEFAULT: '1rem',
      },
    },
    fontFamily: {
      secondary: ['Titillium Web', 'Barlow Condensed', 'sans-serif'],
      primary: ['Manrope', 'sans-serif'],
    },
    extend: {
      colors: {
        background: {
          DEFAULT: 'var(--color-background)',
        },
        foreground: {
          DEFAULT: 'var(--color-foreground)',
        },
        primary: {
          DEFAULT: 'var(--color-primary)',
        },
        secondary: {
          DEFAULT: 'var(--color-secondary)',
        },
      },
    },
  },
  plugins: [],
};
