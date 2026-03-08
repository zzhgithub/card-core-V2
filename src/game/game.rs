use crate::desk::{Deck, DeckConfig, DeckValidationError};
use crate::entity::{CardEntity, CardEntityId, IdGenerator};
use crate::game::game_phase::GamePhase;
use crate::lua_api::lua_api::LuaApi;
use crate::player::{PlayerController, PlayerId, PlayerZones};
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub struct Game {
    pub current_phase: GamePhase,
    pub players: Vec<Box<dyn PlayerController>>,
    pub current_player_id: PlayerId,
    pub card_entities: HashMap<CardEntityId, CardEntity>,
    pub player_decks: Vec<Vec<u64>>,
    pub player_zones: Vec<PlayerZones>,
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
        controller1: Box<dyn PlayerController>,
        controller2: Box<dyn PlayerController>,
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

        let mut card_entities: HashMap<CardEntityId, CardEntity> = HashMap::new();
        let mut player1_entity_ids: Vec<CardEntityId> = Vec::new();
        let mut player2_entity_ids: Vec<CardEntityId> = Vec::new();

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

        // Shuffle the player decks randomly
        let mut rng = rand::thread_rng();
        player1_entity_ids.shuffle(&mut rng);
        player2_entity_ids.shuffle(&mut rng);

        let mut player_zones = vec![PlayerZones::new(5), PlayerZones::new(5)];

        for _ in 0..5 {
            if let Some(card_id) = player1_entity_ids.pop() {
                if let Err(_) = player_zones[0].add_to_hand(card_id) {
                    player1_entity_ids.push(card_id);
                }
            }

            if let Some(card_id) = player2_entity_ids.pop() {
                if let Err(_) = player_zones[1].add_to_hand(card_id) {
                    player2_entity_ids.push(card_id);
                }
            }
        }

        Ok(Self {
            current_phase: GamePhase::Start,
            players: vec![controller1, controller2],
            current_player_id: 0,
            card_entities,
            player_decks: vec![player1_entity_ids, player2_entity_ids],
            player_zones,
            id_generator: id_gen,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?;

        let first_player_index = (start_time.subsec_nanos() & 1) as PlayerId;
        self.current_player_id = first_player_index;

        println!(
            "Starting game with player {} as first player",
            self.current_player_id
        );

        loop {
            match self.current_phase {
                crate::game::game_phase::GamePhase::Start => {
                    println!("Current phase: Start");
                    self.current_phase = crate::game::game_phase::GamePhase::Draw;
                    let action = self.players[self.current_player_id]
                        .ask_player_action(crate::player::AskAction::CurrentPlayer);
                    println!(
                        "Player {} performed action: {:?}",
                        self.current_player_id, action
                    );
                }
                crate::game::game_phase::GamePhase::Draw => {
                    println!("Current phase: Draw");
                    self.current_phase = crate::game::game_phase::GamePhase::Recycle;

                    let action = self.players[self.current_player_id]
                        .ask_player_action(crate::player::AskAction::CurrentPlayer);
                    println!(
                        "Player {} performed action: {:?}",
                        self.current_player_id, action
                    );
                }
                crate::game::game_phase::GamePhase::Recycle => {
                    println!("Current phase: Recycle");
                    self.current_phase = crate::game::game_phase::GamePhase::MainPhase1;

                    let action = self.players[self.current_player_id]
                        .ask_player_action(crate::player::AskAction::CurrentPlayer);
                    println!(
                        "Player {} performed action: {:?}",
                        self.current_player_id, action
                    );
                }
                crate::game::game_phase::GamePhase::MainPhase1 => {
                    println!("Current phase: MainPhase1");
                    self.current_phase = crate::game::game_phase::GamePhase::Battle;

                    let action = self.players[self.current_player_id]
                        .ask_player_action(crate::player::AskAction::CurrentPlayer);
                    println!(
                        "Player {} performed action: {:?}",
                        self.current_player_id, action
                    );
                }
                crate::game::game_phase::GamePhase::Battle => {
                    println!("Current phase: Battle");
                    self.current_phase = crate::game::game_phase::GamePhase::MainPhase2;

                    let action = self.players[self.current_player_id]
                        .ask_player_action(crate::player::AskAction::CurrentPlayer);
                    println!(
                        "Player {} performed action: {:?}",
                        self.current_player_id, action
                    );
                }
                crate::game::game_phase::GamePhase::MainPhase2 => {
                    println!("Current phase: MainPhase2");
                    self.current_phase = crate::game::game_phase::GamePhase::End;

                    let action = self.players[self.current_player_id]
                        .ask_player_action(crate::player::AskAction::CurrentPlayer);
                    println!(
                        "Player {} performed action: {:?}",
                        self.current_player_id, action
                    );
                }
                crate::game::game_phase::GamePhase::End => {
                    println!("Current phase: End game completed");
                    self.current_phase = crate::game::game_phase::GamePhase::GameOver;
                }
                crate::game::game_phase::GamePhase::GameOver => {
                    println!("Game over reached!");
                    break;
                }
            }

            self.current_player_id = (self.current_player_id + 1) % self.players.len();
        }

        Ok(())
    }
}
