use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

/// 卡组验证错误
#[derive(Debug, Clone)]
pub enum DeckValidationError {
    /// 卡组为空
    EmptyDeck,
    /// 卡组数量不足
    InsufficientCards { min: usize, actual: usize },
    /// 卡组数量超过上限
    ExceededMaxCards { max: usize, actual: usize },
    /// 卡片ID不存在
    CardNotFound(String),
    /// 卡片数量超过允许的最大 copies 数
    ExceededMaxCopies {
        card_id: String,
        max: usize,
        actual: usize,
    },
    /// 卡组中缺少必需的核心卡片
    MissingRequiredCard(String),
}

/// 卡组配置
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeckConfig {
    /// 卡组最小数量
    pub min_size: usize,
    /// 卡组最大数量
    pub max_size: usize,
    /// 每张卡片允许的最大 copies 数
    pub max_copies_per_card: usize,
    /// 是否需要核心卡片
    pub require_core_cards: bool,
    /// 核心卡片ID列表
    pub core_card_ids: Vec<String>,
}

impl Default for DeckConfig {
    fn default() -> Self {
        Self {
            min_size: 20,
            max_size: 60,
            max_copies_per_card: 3,
            require_core_cards: false,
            core_card_ids: vec![],
        }
    }
}

/// 卡组结构体
#[derive(Debug, Clone)]
pub struct Deck {
    /// 卡片ID列表
    pub card_ids: Vec<String>,
    /// 卡片ID到数量的映射
    pub card_counts: HashMap<String, usize>,
}

impl Deck {
    /// 创建新的卡组
    pub fn new(card_ids: Vec<String>) -> Self {
        let mut card_counts: HashMap<String, usize> = HashMap::new();
        for id in &card_ids {
            *card_counts.entry(id.clone()).or_insert(0) += 1;
        }
        Self {
            card_ids,
            card_counts,
        }
    }

    /// 验证卡组是否合法
    pub fn validate(
        &self,
        config: &DeckConfig,
        available_card_ids: &[String],
    ) -> Result<(), DeckValidationError> {
        // 检查卡组是否为空
        if self.card_ids.is_empty() {
            return Err(DeckValidationError::EmptyDeck);
        }

        // 检查卡组数量是否足够
        if self.card_ids.len() < config.min_size {
            return Err(DeckValidationError::InsufficientCards {
                min: config.min_size,
                actual: self.card_ids.len(),
            });
        }

        // 检查卡组数量是否超过上限
        if self.card_ids.len() > config.max_size {
            return Err(DeckValidationError::ExceededMaxCards {
                max: config.max_size,
                actual: self.card_ids.len(),
            });
        }

        // 检查每张卡片的 copies 数是否超过允许的最大值
        for (card_id, count) in &self.card_counts {
            if *count > config.max_copies_per_card {
                return Err(DeckValidationError::ExceededMaxCopies {
                    card_id: card_id.clone(),
                    max: config.max_copies_per_card,
                    actual: *count,
                });
            }
        }

        // 检查所有卡片ID是否存在于可用卡片列表中
        let available_set: std::collections::HashSet<&String> = available_card_ids.iter().collect();
        for card_id in &self.card_ids {
            if !available_set.contains(card_id) {
                return Err(DeckValidationError::CardNotFound(card_id.clone()));
            }
        }

        // 如果需要核心卡片，检查是否包含
        if config.require_core_cards {
            for core_id in &config.core_card_ids {
                if !self.card_counts.contains_key(core_id) {
                    return Err(DeckValidationError::MissingRequiredCard(core_id.clone()));
                }
            }
        }

        Ok(())
    }

    /// 获取卡组中的卡片数量
    pub fn len(&self) -> usize {
        self.card_ids.len()
    }

    /// 检查卡组是否为空
    pub fn is_empty(&self) -> bool {
        self.card_ids.is_empty()
    }
}

pub struct DeckLoader;

impl DeckLoader {
    /// 加载所有卡组定义文件
    pub fn load_decks(desk_directory: &str) -> Result<HashMap<String, Vec<String>>, io::Error> {
        let mut decks: HashMap<String, Vec<String>> = HashMap::new();
        let path = Path::new(desk_directory);

        if !path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Desk directory does not exist",
            ));
        }

        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                    let card_ids = Self::read_deck_from_file(&path)?;
                    decks.insert(file_stem.to_string(), card_ids);
                }
            }
        }

        Ok(decks)
    }

    /// 从单个文件中读取卡组定义
    fn read_deck_from_file(file_path: &Path) -> Result<Vec<String>, io::Error> {
        let content = fs::read_to_string(file_path)?;
        let mut card_ids = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if !line.is_empty() {
                card_ids.push(line.to_string());
            }
        }

        Ok(card_ids)
    }

    /// 通过名称获取卡组
    pub fn get_deck_by_name(
        deck_directory: &str,
        deck_name: &str,
    ) -> Result<Vec<String>, DeckLoadingError> {
        let file_path = Path::new(deck_directory).join(format!("{}.txt", deck_name));

        if !file_path.exists() {
            return Err(DeckLoadingError::DeckNotFound(deck_name.to_string()));
        }

        let card_ids =
            Self::read_deck_from_file(&file_path).map_err(|_| DeckLoadingError::IoError)?;

        Ok(card_ids)
    }
}

#[derive(Debug)]
pub enum DeckLoadingError {
    IoError,
    DeckNotFound(String),
    CardValidationError { card_id: String }, // 当卡片ID不存在时的错误
}
