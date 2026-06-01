import { defineConfig } from "vitepress";
import { withMermaid } from "vitepress-plugin-mermaid";

export default withMermaid(defineConfig({
  title: "Canon",
  description:
    "Semantic governance runtime for AI-assisted engineering work: governed packets, evidence, approvals, lineage, and project memory.",

  cleanUrls: true,
  lastUpdated: true,
  ignoreDeadLinks: true,

  head: [
    ["meta", { name: "theme-color", content: "#070412" }],
    ["link", { rel: "icon", href: "/images/canon-icon.svg" }],
    ["link", { rel: "stylesheet", href: "https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.4.2/css/all.min.css" }]
  ],

  themeConfig: {
    logo: "/images/canon-icon.svg",

    nav: [
      { text: "Guide", link: "/guide/introduction" },
      { text: "Governance", link: "/governance/governed-packets" },
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
            { text: "Constitution", link: "/guide/constitution" },
            { text: "Canon Modes", link: "/guide/canon-modes" },
            { text: "Getting Started", link: "/guide/getting-started" },
            { text: "Installation", link: "/guide/installation" },
            { text: "Core Concepts", link: "/guide/core-concepts" },
            { text: "First Packet", link: "/guide/first-packet" },
            { text: "Common Workflows", link: "/guide/common-workflows" }
          ]
        }
      ],
      "/governance/": [
        {
          text: "Governance",
          items: [
            { text: "Governed Packets", link: "/governance/governed-packets" },
            { text: "Evidence", link: "/governance/evidence" },
            { text: "Approvals", link: "/governance/approvals" },
            { text: "Lineage", link: "/governance/lineage" },
            { text: "Project Memory", link: "/governance/project-memory" },
            { text: "Provenance", link: "/governance/provenance" },
            { text: "Accessibility And Alt Text", link: "/governance/accessibility-alt-text" },
            { text: "Skill Runtime Contracts", link: "/governance/skill-runtime-contracts" }
          ]
        }
      ],
      "/skills/": [
        {
          text: "Skills",
          items: [
            { text: "Overview", link: "/skills/overview" },
            { text: "Contracts", link: "/skills/contracts" },
            { text: "Preflight", link: "/skills/preflight" },
            { text: "Hook Traces", link: "/skills/hook-traces" },
            { text: "AI Provenance", link: "/skills/ai-provenance" },
            { text: "Runtime Boundaries", link: "/skills/runtime-boundaries" }
          ]
        }
      ],
      "/architecture/": [
        {
          text: "Architecture",
          items: [
            { text: "Runtime Model", link: "/architecture/runtime-model" },
            { text: "Packet Lifecycle", link: "/architecture/packet-lifecycle" },
            { text: "Approval Lifecycle", link: "/architecture/approval-lifecycle" },
            { text: "Lineage Model", link: "/architecture/lineage-model" },
            { text: "Boundline Integration", link: "/architecture/boundline-integration" }
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
