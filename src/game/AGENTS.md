# GAME MODULE KNOWLEDGE BASE

**Generated:** 2026-03-08
**Commit:** $(git rev-parse --short HEAD)
**Branch:** $(git rev-parse --abbrev-ref HEAD)

## OVERVIEW
Core game state management, game flow control, and phase transition module.

## STRUCTURE
```
src/game/
├── game.rs         # Main Game struct and core game state
├── game_phase.rs   # Game phase definitions and transitions
└── mod.rs          # Module exports
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| Game structure | src/game/game.rs | Game struct and initialization logic |
| Phase definitions | src/game/game_phase.rs | Game phase types and transition flow |
| State transitions | src/game/game_phase.rs | Valid phase transition rules |

## CONVENTIONS
- 9 distinct game phases in specific order
- Player zones: Desktop, Hand, ForEnd, BackEnd, CostZone, Grave, HP
- Payment system: CostZone cards + RealPoints to pay costs
- Effects processed in a stack with LIFO execution

## ANTI-PATTERNS (THIS MODULE)
- DO NOT bypass phase transition validation rules
- DO NOT allow invalid player actions outside current phase
- Avoid modifying game state without proper validation

## UNIQUE STYLES
- Phase-based architecture with state validation at each transition
- Stack-based effect execution system (LIFO)
- Zone-dependent mechanics (Front-back stage positioning)

## COMMANDS
```bash
# Test game mechanics and phase transitions
cargo test -p card-ai -- game
```

## NOTES
- Each game has 2 players with distinct zones
- Deck size must remain 40-60 cards throughout gameplay
- RealPoints system affects combat and payment mechanics
- Win/lose conditions checked at predetermined checkpoints