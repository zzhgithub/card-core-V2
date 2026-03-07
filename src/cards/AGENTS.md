# CARDS MODULE KNOWLEDGE BASE

**Generated:** 2026-03-08
**Commit:** $(git rev-parse --short HEAD)
**Branch:** $(git rev-parse --abbrev-ref HEAD)

## OVERVIEW
Core card definitions, data structures, and card entity management module.

## STRUCTURE
```
src/cards/
├── card_def.rs     # Card data types, enums, and CardBuilder
└── mod.rs          # Module exports
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Card types/attributes | src/cards/card_def.rs | CardType, CardAttribute, CardCategory enums |
| Card builder | src/cards/card_def.rs | CardBuilder for constructing card entities |
| Card data structures | src/cards/card_def.rs | Core card struct definitions |

## CONVENTIONS
- CardType includes Character, Strategy, Item, Legend cards
- CardAttribute uses tripartite system: Rationality, Divinity, Spirituality
- CardCategory: Math, Science, Art, Philosophy, Mystery
- Effects mapped as HashMap<String, Effect> with "e1", "e2", etc. keys

## ANTI-PATTERNS (THIS MODULE)
- DO NOT create cards without proper attribute/category assignment
- DO NOT bypass CardBuilder for creating card instances
- Avoid hardcoded card fields outside CardBuilder pattern

## UNIQUE STYLES
- Chained CardBuilder pattern for construction
- Enums with localized Chinese descriptions
- Card ID format: S[pack_no]-[card_type]-[card_no]

## COMMANDS
```bash
# Test card creation and validation
cargo test -p card-ai -- cards
```

## NOTES
- Card IDs follow S###-X-### format (e.g. S001-C-001)
- Characters have attack attribute; Strategies have special properties; Items have storage properties
- All card validations are handled through builder pattern