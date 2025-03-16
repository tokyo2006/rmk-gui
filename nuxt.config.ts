// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',
  future: {
    compatibilityVersion: 4,
  },
  devtools: { enabled: true },
  ssr: false,
  devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },

  imports: {
    presets: [
      {
        from: '@tauri-apps/api/core',
        imports: ['invoke'],
      },
    ],
  },

  app: {
    pageTransition: { name: 'page', mode: 'out-in' },
  },

  typescript: {
    typeCheck: true,
  },

  css: ['~/assets/main.css'],
  tailwindcss: {
    cssPath: '~/assets/main.css',
  },
  shadcn: {
    prefix: '',
    componentDir: './app/components/ui',
  },
  colorMode: {
    classSuffix: '',
  },

  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
    },
  },

  modules: ['@nuxtjs/tailwindcss', '@nuxtjs/color-mode', '@pinia/nuxt', 'shadcn-nuxt', '@nuxt/icon'],
});
