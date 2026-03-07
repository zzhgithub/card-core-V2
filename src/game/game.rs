use crate::desk::{Deck, DeckConfig, DeckValidationError};
use crate::entity::{CardEntity, IdGenerator};
use crate::game::game_phase::GamePhase;
use crate::lua_api::lua_api::LuaApi;
use crate::player::{PlayerController, PlayerId};
use std::collections::HashMap;

pub struct Game {
    pub current_phase: GamePhase,
    pub players: Vec<Box<dyn PlayerController>>,
    pub current_player_id: PlayerId,
    pub card_entities: HashMap<u64, CardEntity>,
    pub player_decks: Vec<Vec<u64>>,
    id_generator: IdGenerator,
}

#[derive(Debug)]
pub enum GameNewError {
    DeckValidation(DeckValidationError),
    CardNotFound(String),
}

impl Game {
    pub fn new(
        player1_deck_ids: Vec<String>,
        player2_deck_ids: Vec<String>,
        lua_api: &LuaApi,
        id_generator: IdGenerator,
        deck_config: &DeckConfig,
    ) -> Result<Self, GameNewError> {
        let mut id_gen = id_generator;

        let player1_deck = Deck::new(player1_deck_ids.clone());
        let player2_deck = Deck::new(player2_deck_ids.clone());

        let available_card_ids: Vec<String> = lua_api.card_definitions.keys().cloned().collect();

        player1_deck
            .validate(deck_config, &available_card_ids)
            .map_err(GameNewError::DeckValidation)?;

        player2_deck
            .validate(deck_config, &available_card_ids)
            .map_err(GameNewError::DeckValidation)?;

        let mut card_entities: HashMap<u64, CardEntity> = HashMap::new();
        let mut player1_entity_ids: Vec<u64> = Vec::new();
        let mut player2_entity_ids: Vec<u64> = Vec::new();

        for card_id in &player1_deck.card_ids {
            let card_def = lua_api
                .card_definitions
                .get(card_id)
                .ok_or_else(|| GameNewError::CardNotFound(card_id.clone()))?;
            let entity_id = id_gen.generate();
            let entity = CardEntity::new(entity_id, card_def.clone());
            player1_entity_ids.push(entity_id);
            card_entities.insert(entity_id, entity);
        }

        for card_id in &player2_deck.card_ids {
            let card_def = lua_api
                .card_definitions
                .get(card_id)
                .ok_or_else(|| GameNewError::CardNotFound(card_id.clone()))?;
            let entity_id = id_gen.generate();
            let entity = CardEntity::new(entity_id, card_def.clone());
            player2_entity_ids.push(entity_id);
            card_entities.insert(entity_id, entity);
        }

        Ok(Self {
            current_phase: GamePhase::Start,
            players: vec![],
            current_player_id: 0,
            card_entities,
            player_decks: vec![player1_entity_ids, player2_entity_ids],
            id_generator: id_gen,
        })
    }
}
