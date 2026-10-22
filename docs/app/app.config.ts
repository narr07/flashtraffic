export default defineAppConfig({
  ui: {
    colors: {
      primary: 'green',
      neutral: 'zinc'
    },
    footer: {
      slots: {
        root: 'border-t border-default',
        left: 'text-sm text-muted'
      }
    }
  },
  seo: {
    siteName: 'FlashTraffic Documentation'
  },
  header: {
    title: '',
    to: '/',
    logo: {
      alt: 'FlashTraffic',
      light: '',
      dark: ''
    },
    search: false,
    colorMode: true,
    links: [{
      'icon': 'i-simple-icons-github',
      'to': 'https://github.com/narr07/flashtraffic',
      'target': '_blank',
      'aria-label': 'FlashTraffic on GitHub'
    }]
  },
  footer: {
    credits: `FlashTraffic • Open-Source Web Traffic Simulator © ${new Date().getFullYear()}`,
    colorMode: false,
    links: [{
      'icon': 'i-simple-icons-github',
      'to': 'https://github.com/narr07/flashtraffic',
      'target': '_blank',
      'aria-label': 'FlashTraffic on GitHub'
    }]
  },
  toc: {
    title: 'On this page',
    bottom: {
      title: 'Repository',
      edit: 'https://github.com/narr07/flashtraffic',
      links: [{
        icon: 'i-lucide-star',
        label: 'Star on GitHub',
        to: 'https://github.com/narr07/flashtraffic',
        target: '_blank'
      }]
    }
  }
})
