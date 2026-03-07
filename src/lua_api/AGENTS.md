# LUA_API MODULE KNOWLEDGE BASE

**Generated:** 2026-03-08
**Commit:** $(git rev-parse --short HEAD)
**Branch:** $(git rev-parse --abbrev-ref HEAD)

## OVERVIEW
Module handling Lua scripting integration and card definition loading from Lua files.

## STRUCTURE
```
src/lua_api/
├── lua_api.rs      # Main Lua API wrapper and interface
├── load_cards.rs   # Card loading functions from Lua scripts
├── config.rs       # Lua configuration and script directory settings
└── mod.rs          # Module exports
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Lua initialization | src/lua_api/lua_api.rs | Lua state creation and setup |
| Card loading | src/lua_api/load_cards.rs | Parse Lua card definitions and create Rust card structs |
| Script configuration | src/lua_api/config.rs | Directory paths and Lua script settings |

## CONVENTIONS
- Use mlua crate for all Lua operations
- Card effects stored as HashMap with keys "e1", "e2", etc.
- Lua scripts must follow specific card definition patterns
- Handle Lua errors gracefully without panicking

## ANTI-PATTERNS (THIS MODULE)
- DO NOT bypass the Lua API without proper error handling
- DO NOT mix Rust logic with Lua-defined behavior unnecessarily
- Avoid complex logic directly in Lua - keep in Rust instead

## UNIQUE STYLES
- Hybrid approach with Rust safety wrapping Lua flexibility
- String-based effect key mapping (e1, e2, e3...)
- Schema validation on loaded Lua data

## COMMANDS
```bash
# Test Lua integration
cargo test -p card-ai -- lua_api
# Run specific Lua integration tests
```

## NOTES
- Lua scripts define card behavior but should not contain complex business logic
- All card validation still happens in Rust after Lua loads
- Memory managed primarily by Rust with garbage collection for Lua side