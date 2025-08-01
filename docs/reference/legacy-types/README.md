# Legacy MCP Type Definitions

This directory contains legacy type definitions that were used during the schema upgrade process from 2025-03-26 to 2025-06-18 specification.

## Files

- **mcp_2025_complete_messages.rs** - Complete MCP message types for 2025-03-26 specification
- **mcp_2025_complete_types.rs** - Complete MCP core types for 2025-03-26 specification

## Purpose

These files are preserved for:
- **Historical Reference** - Understanding the evolution of MCP types 
- **Developer Reference** - Comparing old vs new implementations
- **Migration Documentation** - Showing what changed during the upgrade

## Status

⚠️ **DEPRECATED** - These files are no longer used in the active codebase. The current implementation uses the types defined in `src/protocol/types.rs` and `src/protocol/messages.rs` which comply with the 2025-06-18 specification.

## Current Implementation

For current MCP protocol implementation, see:
- `src/protocol/types.rs` - Current protocol types (2025-06-18)
- `src/protocol/messages.rs` - Current protocol messages (2025-06-18)
- `docs/mcp-schema-2025-06-18.json` - Current JSON schema reference

## Schema Upgrade

The schema upgrade from 2025-03-26 to 2025-06-18 was completed successfully with:
- ✅ 100% backward compatibility maintained
- ✅ All enhanced features implemented
- ✅ Zero breaking changes to existing APIs

For details, see:
- `docs/UPGRADE_COMPLETE.md` - Complete upgrade summary
- `docs/UPGRADE_PROGRESS.md` - Technical upgrade details
