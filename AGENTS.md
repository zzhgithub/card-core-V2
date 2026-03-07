# PROJECT KNOWLEDGE BASE

**Generated:** 2026-03-08
**Commit:** $(git rev-parse --short HEAD)
**Branch:** $(git rev-parse --abbrev-ref HEAD)

## OVERVIEW
Card-Game application built in Rust with Lua card definition integration and structured game flow system.

## STRUCTURE
```
card-ai/
├── Cargo.toml          # Rust project manifest (edition 2024)
├── Cargo.lock          # Dependency lock file
├── src/                # Rust source code
│   ├── cards/          # Card definitions and logic 
│   ├── game/           # Game phase management
│   ├── effect/         # Card effect processing
│   ├── entity/         # Entity management
│   ├── lua_api/        # Lua integration layer
│   ├── player/         # Player actions
│   └── desk/           # Deck management
├── lua/                # Lua card definition scripts
├── desks/              # Deck configuration files
├── lua_api/            # Alternative path - check if duplicate?
├── config.toml         # Runtime configuration
└── AGENTS.md           # This knowledge base
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Card definitions | src/cards/card_def.rs | Contains CardType, CardAttribute, CardCategory enums |
| Game flow | src/game/game.rs, src/game/game_phase.rs | Core game logic and phase transitions |
| Lua integration | src/lua_api/ | Scripts, API, and card loading from Lua files |
| Card effects | src/effect/effect_def.rs | Effect structures and action definitions |
| Deck management | src/desk/deck.rs | Card loading, validation and deck structure |
| Player actions | src/player/ | Decision making and interaction flows |

## CODE MAP
| Symbol | Type | Location | Role |
|--------|------|----------|------|
| CardBuilder | struct | src/cards/card_def.rs | Constructs card objects from data |
| Game | struct | src/game/game.rs | Core game state and engine |
| LuaApi | struct | src/lua_api/lua_api.rs | Lua interpreter interface |
| Effect | struct | src/effect/effect_def.rs | Represents card effect behaviors |

## CONVENTIONS
- CardType includes: Character, Strategy, Item, Legend cards
- CardAttribute: Rationality, Divinity, Spirituality (triad system)  
- CardCategory: Math, Science, Art, Philosophy, Mystery
- Cards defined in Lua files use e1, e2 keys for effects in HashMap
- Edition 2024 required (non-standard Rust edition)
- ID format: S[pack_no]-[card_type]-[card_no]; e.g. S001-C-001

## ANTI-PATTERNS (THIS PROJECT)
- No unsafe Rust usage encouraged
- DO NOT use edition 2021 or older (use 2024)
- DO NOT bypass Lua layer for card definitions
- Avoid panic!/unwrap() in core game operations

## UNIQUE STYLES
- Lua-Rust hybrid architecture
- Heavy use of enums for card mechanics
- Card effect system with HashMap(key=E1,E2..)
- Chinese comments in enum definitions

## COMMANDS
```bash
# Build project
cargo build

# Code checks
cargo check && cargo clippy --all-targets --all-features

# Formatting
cargo fmt && rustfmt src/**/*.rs --edition 2021

# Testing
cargo test
```

## NOTES
- Lua scripts store card definitions in /lua directory
- Lua effects accessed through mlua crate with "e1", "e2" pattern
- Game has 9 distinct phases with specific validation rules
- Deck size must be 40-60 cards with max 3 copies of any card