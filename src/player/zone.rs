use crate::entity::CardEntityId;
use serde::{Deserialize, Serialize};

/// Constants for player zone limits
pub const HAND_LIMIT: usize = 20;
pub const FRONT_LINE_SIZE: usize = 5;
pub const BACK_LINE_SIZE: usize = 5;
pub const COST_ZONE_LIMIT: usize = 6;

/// Represents a zone that can hold multiple card entities (allows stacking in the future)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Zone {
    /// Vector of card entity IDs in this zone
    cards: Vec<CardEntityId>,
    /// Maximum capacity of the zone (initially restricted for position limits)
    capacity: Option<usize>,
}

impl Zone {
    /// Create a new zone with unlimited capacity
    pub fn new_unlimited() -> Self {
        Self {
            cards: Vec::new(),
            capacity: None,
        }
    }

    /// Create a new zone with a fixed capacity
    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            cards: Vec::with_capacity(capacity),
            capacity: Some(capacity),
        }
    }

    /// Add a card entity ID to the zone if capacity allows
    pub fn add_card(&mut self, card_id: CardEntityId) -> Result<(), &'static str> {
        if let Some(max_capacity) = self.capacity {
            if self.cards.len() >= max_capacity {
                return Err("Zone limit reached");
            }
        }

        self.cards.push(card_id);
        Ok(())
    }

    /// Remove a card from the zone
    pub fn remove_card(&mut self, card_id: CardEntityId) -> bool {
        if let Some(pos) = self.cards.iter().position(|&id| id == card_id) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get immutable reference to cards in this zone
    pub fn cards(&self) -> &Vec<CardEntityId> {
        &self.cards
    }

    /// Get mutable reference to cards in this zone
    pub fn cards_mut(&mut self) -> &mut Vec<CardEntityId> {
        &mut self.cards
    }

    /// Get the number of cards in the zone
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Check if the zone is empty
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Clear all cards from the zone
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Get the capacity of the zone (None if unlimited)
    pub fn capacity(&self) -> Option<usize> {
        self.capacity
    }

    /// Set a new capacity for the zone
    pub fn set_capacity(&mut self, capacity: usize) {
        self.capacity = Some(capacity);
    }
}

/// Represents all zones for a single player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerZones {
    /// Hand containing cards the player can play
    pub hand: Vec<CardEntityId>,

    /// Front line battlefield positions (5 slots for combat)
    pub front_line: Vec<Zone>,

    /// Back line battlefield positions (5 slots for support)  
    pub back_line: Vec<Zone>,

    /// Cost zone - where cards are placed to pay for effects/summons
    pub cost_zone: Vec<CardEntityId>,

    /// Graveyard - where destroyed/sacrificed cards go
    pub graveyard: Vec<CardEntityId>,

    /// Hit Points representing player life
    pub hp: u32,

    /// Real Points for special abilities/payment in lieu of cards
    pub real_points: u32,
}

impl PlayerZones {
    /// Create a new PlayerZones with empty zones and default values
    pub fn new(initial_hp: u32) -> Self {
        Self {
            hand: Vec::new(),
            front_line: (0..FRONT_LINE_SIZE)
                .map(|_| Zone::new_with_capacity(1))
                .collect(),
            back_line: (0..BACK_LINE_SIZE)
                .map(|_| Zone::new_with_capacity(1))
                .collect(),
            cost_zone: Vec::new(),
            graveyard: Vec::new(),
            hp: initial_hp,
            real_points: 0,
        }
    }

    /// Add a card to hand, respecting the maximum hand size
    pub fn add_to_hand(&mut self, card_entity_id: CardEntityId) -> Result<(), &'static str> {
        if self.hand.len() >= HAND_LIMIT {
            return Err("Hand limit reached");
        }

        self.hand.push(card_entity_id);
        Ok(())
    }

    /// Try to add a card to front line, finding the first available position
    pub fn add_to_front_line(&mut self, card_entity_id: CardEntityId) -> Result<(), &'static str> {
        for zone in &mut self.front_line {
            if zone.add_card(card_entity_id).is_ok() {
                return Ok(());
            }
        }
        Err("No available position in front line")
    }

    /// Add a specific card to a specific front line position (0-indexed)
    pub fn add_to_front_line_position(
        &mut self,
        position: usize,
        card_entity_id: CardEntityId,
    ) -> Result<(), &'static str> {
        if position >= self.front_line.len() {
            return Err("Invalid front line position");
        }
        self.front_line[position].add_card(card_entity_id)
    }

    /// Try to add a card to back line, finding the first available position
    pub fn add_to_back_line(&mut self, card_entity_id: CardEntityId) -> Result<(), &'static str> {
        for zone in &mut self.back_line {
            if zone.add_card(card_entity_id).is_ok() {
                return Ok(());
            }
        }
        Err("No available position in back line")
    }

    /// Add a specific card to a specific back line position (0-indexed)
    pub fn add_to_back_line_position(
        &mut self,
        position: usize,
        card_entity_id: CardEntityId,
    ) -> Result<(), &'static str> {
        if position >= self.back_line.len() {
            return Err("Invalid back line position");
        }
        self.back_line[position].add_card(card_entity_id)
    }

    /// Add a card to cost zone, respecting the limit
    pub fn add_to_cost_zone(&mut self, card_entity_id: CardEntityId) -> Result<(), &'static str> {
        if self.cost_zone.len() >= COST_ZONE_LIMIT {
            return Err("Cost zone limit reached");
        }

        self.cost_zone.push(card_entity_id);
        Ok(())
    }

    /// Move a card from any location to graveyard
    pub fn move_to_graveyard(&mut self, card_entity_id: CardEntityId) {
        // Remove from hand if present
        if let Some(pos) = self.hand.iter().position(|&id| id == card_entity_id) {
            self.hand.remove(pos);
        }

        // Remove from front line if present
        for zone in &mut self.front_line {
            if zone.remove_card(card_entity_id) {
                break;
            }
        }

        // Remove from back line if present
        for zone in &mut self.back_line {
            if zone.remove_card(card_entity_id) {
                break;
            }
        }

        // Remove from cost zone if present
        if let Some(pos) = self.cost_zone.iter().position(|&id| id == card_entity_id) {
            self.cost_zone.remove(pos);
        }

        self.graveyard.push(card_entity_id);
    }

    /// Get total card count across all zones except deck/discard
    pub fn card_count(&self) -> usize {
        let front_count: usize = self.front_line.iter().map(|zone| zone.len()).sum();
        let back_count: usize = self.back_line.iter().map(|zone| zone.len()).sum();

        self.hand.len() + front_count + back_count + self.cost_zone.len() + self.graveyard.len()
    }
}
