pub mod card_entity;
pub mod id_generator;

pub use card_entity::CardEntity;
pub use id_generator::IdGenerator;

/// Type alias for card entity ID
pub type CardEntityId = u64;
