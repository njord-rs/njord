import colors from 'tailwindcss/colors';

/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
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
          DEFAULT: colors.zinc['900'],
        },
        foreground: {
          DEFAULT: colors.zinc['100'],
        },
        primary: {
          DEFAULT: '#78350f',
        },
        secondary: {
          DEFAULT: colors.zinc['800'],
        },
      },
    },
  },
  plugins: [],
};
