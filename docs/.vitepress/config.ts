import { defineConfig } from "vitepress";
import { withMermaid } from "vitepress-plugin-mermaid";

export default withMermaid(defineConfig({
  base: "/canon/",
  title: "Canon",
  description:
    "Governance and Context Intelligence for AI-assisted software delivery.",

  cleanUrls: true,
  lastUpdated: true,
  ignoreDeadLinks: true,

  head: [
    ["meta", { name: "theme-color", content: "#070412" }],
    // ["link", { rel: "icon", href: "/images/canon-icon.svg" }],
    ["link", { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css" }]
  ],

  themeConfig: {
    // logo: "/images/canon-icon.svg",

    nav: [
      { text: "Guide", link: "/guide/introduction" },
      { text: "Architecture", link: "/architecture/runtime-model" },
      { text: "Reference", link: "/reference/cli" },
      { text: "Roadmap", link: "/roadmap/" },
      { text: "GitHub", link: "https://github.com/apply-the/canon" }
    ],

    sidebar: {
      "/guide/": [
        {
          text: "Guide",
          items: [
            { text: "Introduction", link: "/guide/introduction" },
            { text: "Installation", link: "/guide/installation" },
            { text: "Quickstart", link: "/guide/quickstart" },
            { text: "Getting Started", link: "/guide/getting-started" },
            { text: "First Workspace", link: "/guide/first-workspace" },
            { text: "Core Concepts", link: "/guide/core-concepts" },
            { text: "Common Workflows", link: "/guide/common-workflows" },
            { text: "Constitution", link: "/guide/constitution" }
          ]
        }
      ],
      "/governance/": [
        {
          text: "Governance",
          items: [
            { text: "Guardians", link: "/governance/guardians" }
          ]
        }
      ],
      "/architecture/": [
        {
          text: "Architecture",
          items: [
            { text: "Runtime Model", link: "/architecture/runtime-model" },
            { text: "Session Model", link: "/architecture/session-model" },
            { text: "Context Intelligence", link: "/architecture/context-intelligence" },
            { text: "Persistent Stdio", link: "/architecture/persistent-stdio" },
            { text: "Recursive Stage Refinement", link: "/architecture/recursive-stage-refinement" },
            { text: "Security Model", link: "/architecture/security-model" }
          ]
        }
      ],
      "/reference/": [
        {
          text: "Reference",
          items: [
            { text: "CLI Reference", link: "/reference/cli" },
            { text: "Configuration", link: "/reference/configuration" },
            { text: "File Layout", link: "/reference/file-layout" },
            { text: "Glossary", link: "/reference/glossary" },
            { text: "FAQ", link: "/reference/faq" }
          ]
        }
      ]
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/apply-the/canon" }
    ],

    search: {
      provider: "local"
    },

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © Apply The"
    }
  }
}));
