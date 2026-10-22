// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: [
    '@nuxt/eslint',
    '@nuxt/image',
    '@nuxt/ui',
    '@nuxt/content'
  ],

  devtools: {
    enabled: false
  },

  app: {
    baseURL: '/flashtraffic/',
    head: {
      title: 'FlashTraffic - Web Traffic Simulator & Analytics Testing Desktop Platform',
      meta: [
        { name: 'description', content: 'Modern desktop web traffic simulation, load testing, and analytics attribution platform for Windows.' }
      ],
      link: [
        { rel: 'icon', type: 'image/png', href: '/flashtraffic/icon.png' }
      ]
    }
  },

  css: ['~/assets/css/main.css'],

  content: {
    build: {
      markdown: {
        toc: {
          searchDepth: 1
        }
      }
    }
  },

  compatibilityDate: '2026-06-30'
})
