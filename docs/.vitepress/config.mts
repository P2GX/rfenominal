import { defineConfig } from 'vitepress'


export default defineConfig({
  title: "Fenominal",
  description: "HPO-based text mining in Rust and Python",
  base: '/fenominal/',
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' }
    ],

    sidebar: [
      {
        text: 'Introduction',
        items: [
          { text: 'Getting Started', link: '/guide/getting-started' },
          { text: 'Python Usage', link: '/guide/python-api' },
          { text: 'Rust Usage', link: '/guide/rust-api' }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/P2GX/fenominal' }
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026'
    }
  }
})