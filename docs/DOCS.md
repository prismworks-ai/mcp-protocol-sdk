# Documentation Manifest - MCP Protocol SDK

**Last Updated**: 2025-07-31 (Updated with schema upgrade documentation)

## AI Context System Files
- .aicontext/state.json - Current AI context and focus
- .aicontext/tasks.json - AI task management 
- .aicontext/plan.json - AI project planning
- .aicontext/AI-RULES.md - AI operational guidelines
- .aicontext/LOG.md - AI session history

## Core Documentation
- **Project README**: ../README.md - Main project documentation and quick start
- **Documentation Index**: docs/README.md - Documentation hub and navigation
- **Getting Started**: docs/getting-started.md - Installation and basic setup guide
- **Architecture**: docs/architecture.md - System design and component overview
- **Implementation Guide**: docs/implementation-guide.md - Detailed technical implementation overview
- **Configuration**: docs/configuration.md - Configuration options and settings
- **Transports**: docs/transports.md - Transport layer documentation (STDIO, HTTP, WebSocket)
- **SDK Comparison**: docs/comparison-official-sdk.md - Detailed comparison with official MCP SDKs
- **Naming Rationale**: docs/naming-rationale.md - Explanation of our naming choices

## API & Reference
- **API Reference**: docs/api-reference.md - Complete API documentation
- **API Details**: docs/api/README.md - Detailed API specifications
- **Schema Compliance**: docs/SCHEMA_COMPLIANCE.md - MCP protocol compliance details
- **Utils**: docs/utils.md - Utility functions and helpers

## Guides & Examples
- **Examples**: docs/examples.md - Code examples and usage patterns
- **Implementation Guide**: docs/implementation-guide.md - Complete client and server development guide

## Integrations
- **Claude Desktop**: docs/integrations/claude-desktop.md - Claude Desktop integration
- **Cursor IDE**: docs/integrations/cursor.md - Cursor IDE integration
- **Other Clients**: docs/integrations/other-clients.md - Third-party client integrations

## Project Management
- **Development Workflow**: docs/DEVELOPMENT_WORKFLOW.md - Development process and CI/CD setup
- **Repository Hygiene**: docs/REPOSITORY_HYGIENE.md - Guide for maintaining clean public/private file separation
- **Contributing**: docs/CONTRIBUTING.md - Contribution guidelines (moved 2025-07-31 15:13)
- **Changelog**: docs/CHANGELOG.md - Version history and changes (moved 2025-07-31 15:13)
- **Publishing**: docs/PUBLISHING.md - Release and publishing process (moved 2025-07-31 15:13)
- **Publishing Guide**: docs/publishing-guide.md - Detailed publishing instructions
- **Security**: docs/SECURITY.md - Security policies and reporting (moved 2025-07-31 15:13)
- **Local CI Guide**: docs/LOCAL_CI_GUIDE.md - Local development and CI setup (moved 2025-07-31 15:13)
- **OSS Launch Summary**: docs/OSS-LAUNCH-SUMMARY.md - Open source launch details (moved 2025-07-31 15:13)
- **Ecosystem Summary**: docs/ECOSYSTEM_SUMMARY.md - Community and ecosystem information

## Schema Upgrade Documentation
- **Upgrade Complete**: docs/UPGRADE_COMPLETE.md - Complete schema upgrade summary (2025-06-18 specification)
- **Upgrade Progress**: docs/UPGRADE_PROGRESS.md - Detailed upgrade progress and technical details

## Schema References
- **Current Schema**: docs/mcp-schema-2025-06-18.json - JSON schema for current 2025-06-18 specification
- **Legacy Schema**: docs/mcp-schema-2025-03-26.json - JSON schema for previous 2025-03-26 specification
- **Legacy Types**: docs/reference/legacy-types/ - Legacy Rust type definitions (deprecated, for reference only)

## Site Infrastructure
- **Navigation**: docs/nav.md - Site navigation structure
- **Index Page**: docs/index.md - Documentation site homepage
- **404 Page**: docs/404.md - Error page for documentation site
- **Jekyll Config**: docs/_config.yml - Jekyll site configuration
- **Gemfile**: docs/Gemfile - Ruby dependencies for Jekyll site

## Documentation Rules
1. Check this file before creating new docs
2. Update status when docs change
3. Flag docs for review after major changes
4. AI Context system files are in .aicontext/ folder
