use crate::cards::card_def::Card;

#[derive(Debug, Clone)]
pub struct CardEntity {
    pub entity_id: u64,
    pub original_card: Card,
    pub current_card: Card,
}

impl CardEntity {
    pub fn new(entity_id: u64, card: Card) -> Self {
        let original_card = card.clone();
        Self {
            entity_id,
            original_card,
            current_card: card,
        }
    }

    pub fn reset_to_original(&mut self) {
        self.current_card = self.original_card.clone();
    }
}
