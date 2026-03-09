# Effect System Implementation

## Overview
The card game effect system has been implemented with a comprehensive structure that enables cards to define complex behaviors under specific conditions with optional costs and target selections. This system supports both automatic and player-choice-triggered card effects with extensive customization possibilities.

## Core Components

### 1. Effect Structure
The primary structure that represents a card effect with all its aspects:

```rust
pub struct Effect {
    pub trigger: Trigger,                                   // When the effect activates
    pub optional: bool,                                     // Whether player can choose to activate
    pub activation_limit: Option<ActivationLimit>,         // Frequency limits for activation
    pub conditions: Option<Condition>,                     // When the effect can be used (condition tree)
    pub choices: Vec<Choice>,                              // Target selection options
    pub actions: Vec<Action>,                              // Actual effect execution
    pub costs: Option<CostChoice>,                         // Required payments beyond card costs
    pub name: String,                                      // Human-readable name
    pub description: String,                               // Description for display
}
```

### 2. Trigger Enum
Defines when effects are activated during game flow:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Trigger {
    TurnStart(bool),          // bool: true for own turn, false for opponent's turn
    OwnMainPhase,             // During own main phase
    OpponentMainPhase,        // During opponent's main phase
    EitherMainPhase,          // During either player's main phase
    AttackPhase,              // When attacking
    DefensePhase,             // When being attacked
    DamagePhase,              // On damage calculation
    Exposed,                  // When card becomes exposed
    Destroyed,                // When card is destroyed
    Summoned,                 // When card is summoned/played
    EndPhase,                 // At end of turn
    Custom(String),           // For custom trigger conditions
    OnDemand,                 // Effect that can be used at any legal time
}
```

### 3. Activation Limit Enum
Restrictions on how often an effect can be used:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivationLimit {
    OncePerTurn,                    // One use per turn
    OncePerGame,                    // One use per game
    OncePerPlayer(String),          // Once per unique identifier (e.g. card name) per turn
    Limited(u32),                   // Specific number of uses
    PerCombatPhase,                 // Once per combat phase
}
```

### 4. Condition System
Complex conditional logic system with Boolean operators:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    // Basic conditions
    HandCardCountComparison { player: PlayerId, comparison: ComparisonOp, value: u32 },
    FieldCardCountComparison { player: PlayerId, attribute: Option<CardAttribute>, comparison: ComparisonOp, value: u32 },
    CostZoneCardCountComparison { player: PlayerId, attribute: Option<CardAttribute>, comparison: ComparisonOp, value: u32 },
    RealPointComparison { player: PlayerId, comparison: ComparisonOp, value: u32 },
    HealthComparison { player: PlayerId, comparison: ComparisonOp, value: u32 },
    
    // Combined conditions
    And(Box<Condition>, Box<Condition>), // Logical AND
    Or(Box<Condition>, Box<Condition>),  // Logical OR
    Not(Box<Condition>),                 // Logical NOT
    
    // Complex conditions
    CardInZone { player: PlayerId, zone: ZoneType, card_properties: CardPropertyFilter },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ZoneType {
    Hand,
    FieldFront,  // Front-line positions
    FieldBack,   // Back-line positions
    Deck,
    Graveyard,
    CostZone,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardPropertyFilter {
    Attribute(CardAttribute),
    CardType(CardType),
    Name(String),
    SpecificCardId(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonOp {
    Equals(u32),
    GreaterThan(u32),
    LessThan(u32),
    GreaterThanOrEqual(u32),
    LessThanOrEqual(u32),
    NotEquals(u32),
}
```

### 5. Choice System
Defines how targets for effects are selected:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Choice {
    SelectAnyCardFromPlayer { 
        target: PlayerId, 
        zone: ZoneType,
        constraints: Option<CardConstraint>,
        count: u32 // How many cards to select
    },
    SelectSpecificPosition { 
        target: PlayerId, 
        position_range: PositionRange 
    },
    SelectNumberOfCards {
        target: PlayerId,
        min: u32,
        max: u32,
    },
    ChooseAction { 
        options: Vec<String>, 
        allow_multiple: bool 
    },
    SelectAttribute { 
        attributes: Vec<CardAttribute>
    },
    NoChoice, // Effect proceeds without targeting any specific cards
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CardConstraint {
    SpecificAttribute(CardAttribute),
    SpecificType(CardType),
    MinCost(u32),
    MaxCost(u32),
    Damaged, // For destroyed/breaken units
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PositionRange {
    FrontLine,
    BackLine,
    AnyFieldPosition,
    Specific(Vec<usize>), // Specific battlefield indexes
}
```

### 6. Action System
Actual game changes that occur when effects are resolved:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    DrawCard(PlayerId, u32),                      // Draw cards
    DestroyCard(TargetSelector),                        // Destroy selected cards in field
    SendCardToGraveyard(TargetSelector),                // Send cards to graveyard
    AddRealPoint(PlayerId, i32),                  // Add/remove real points
    ChangeCardAttribute(TargetSelector, CardAttribute), // Change card attributes
    Heal(PlayerId, u32),                          // Restore health
    DealDamage(TargetSelector, u32),                    // Cause damage
    MoveCard(TargetSelector, ZoneType, u32),            // Moves card between zones
    ChangeHealth(TargetSelector, i32),                  // Change HP
    GainControl(PlayerId, TargetSelector),        // Gain control of opponent's card
    AddCardToHand(PlayerId, String),              // Add specific card to hand
    DiscardCards(PlayerId, u32),                  // Force discard
    GainLife(PlayerId, u32),                      // Increase max HP
    SetStatus(TargetSelector, StatusModifier),          // Apply status conditions
    Custom(String, Vec<String>),                        // For special case actions
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TargetSelector {
    ChosenInChoice(usize),              // Target from prior choice (refers to choice index)
    AllPlayerCards(PlayerId),           // All cards controlled by player
    TopCards(ZoneType, u32),            // Top N cards of zone
    SpecificCard(u64),                  // Specific card entity
    PreviousActionTargets,              // Targets from previous action in effect sequence
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusModifier {
    CannotAttack,
    CannotBeAttacked,
    DoubleDamage,
    HalfDamage,
    PreventNextDamage(u32),
    CannotPayCost,
}
```

### 7. Cost Choice System
Requirements for activating effects beyond normal card costs:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CostChoice {
    PayCardsFromZone { 
        zone: ZoneType, 
        target: PlayerId, 
        required_count: u32, 
        constraints: Option<CardConstraint> 
    }, // Pay cards from specified zone
    PayRealPoints { amount: u32 }, // Pay real points
    PayLife { amount: u32 },       // Pay life points
    Sacrifice { target: TargetSelector }, // Sacrifice specific cards
    MultipleCosts { options: Vec<Cost> }, // Choose one from multiple cost options
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Cost {
    CardsFromZone { zone: ZoneType, target: PlayerId, count: u32 },
    RealPoints { amount: u32 },
    LifePoints { amount: u32 },
    SpecificCard(u64), // Specific card must be paid/sacrificed
}
```

## Implementation Status
- ✅ Core effect structure with all fields and serialization support
- ✅ All effect components enums (Trigger, Condition, Action, etc.) 
- ✅ Serde annotations for serialization/deserialization support for persistence
- ✅ CardAttribute and CardType enums updated with serialization support
- ✅ Effect system integrated into game architecture with PlayerId type instead of PlayerSelector

## Data Flow
1. Card definitions contain Effect specifications with all components
2. When game reaches a trigger condition, relevant effects are evaluated
3. Check eligibility based on conditions and costs
4. Present options to player if effect is optional
5. Resolve target selections
6. Process any costs
7. Execute action sequence
8. Handle outcome and potential chain reactions

This implementation provides a robust and flexible effect system aligned with the GamePlan requirements document.