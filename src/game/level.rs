use super::board::{Board, ExitPosition};
use super::obstacle::Obstacle;
use super::theme::Theme;
use super::vehicle::Vehicle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldSize {
    Small6x6,
    Medium8x8,
    Big10x10,
}

impl FieldSize {
    pub const ALL: [FieldSize; 3] = [
        FieldSize::Small6x6,
        FieldSize::Medium8x8,
        FieldSize::Big10x10,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            FieldSize::Small6x6 => "Small 6x6",
            FieldSize::Medium8x8 => "Medium 8x8",
            FieldSize::Big10x10 => "Big 10x10",
        }
    }

    #[allow(dead_code)]
    pub fn dimension(&self) -> i32 {
        match self {
            FieldSize::Small6x6 => 6,
            FieldSize::Medium8x8 => 8,
            FieldSize::Big10x10 => 10,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DifficultyTier {
    Relaxed,
    Challenging,
    Hard,
}

impl DifficultyTier {
    pub const ALL: [DifficultyTier; 3] = [
        DifficultyTier::Relaxed,
        DifficultyTier::Challenging,
        DifficultyTier::Hard,
    ];

    pub fn label(&self) -> &'static str {
        match self {
            DifficultyTier::Relaxed => "Relaxed",
            DifficultyTier::Challenging => "Challenging",
            DifficultyTier::Hard => "Hard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PackKey {
    pub size: FieldSize,
    pub difficulty: DifficultyTier,
}

impl PackKey {
    pub const fn new(size: FieldSize, difficulty: DifficultyTier) -> Self {
        Self { size, difficulty }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub id: u32,
    #[serde(default = "default_level_name")]
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub exit: ExitPosition,
    pub vehicles: Vec<Vehicle>,
    #[serde(default)]
    pub obstacles: Vec<Obstacle>,
    #[serde(default = "default_par")]
    pub par_moves: u32,
}

fn default_level_name() -> String {
    "Level".to_string()
}

const fn default_par() -> u32 {
    10
}

impl LevelData {
    pub fn to_board(&self) -> Board {
        let mut board = Board::new(
            self.width,
            self.height,
            self.exit,
            self.vehicles.clone(),
            self.obstacles.clone(),
        );
        board.theme = Theme::for_level(self.id);
        board
    }

    /// Computes earned stars (1..=3) based on moves taken versus par moves.
    pub fn calculate_stars(&self, moves: u32) -> u8 {
        if moves <= self.par_moves {
            3
        } else if moves <= self.par_moves + (self.par_moves / 2).max(2) {
            2
        } else {
            1
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LevelRecord {
    pub completed: bool,
    pub best_moves: Option<u32>,
    pub stars: u8,
}

impl LevelRecord {
    pub fn record_win(&mut self, moves: u32, stars: u8) {
        self.completed = true;
        self.stars = self.stars.max(stars);
        self.best_moves = Some(self.best_moves.map_or(moves, |m| m.min(moves)));
    }
}

/// Mixes levels from multiple difficulty buckets in round-robin zig-zag manner,
/// renumbering the resulting levels sequentially.
pub fn mix_zigzag(buckets: &[Vec<LevelData>]) -> Vec<LevelData> {
    let max_len = buckets.iter().map(Vec::len).max().unwrap_or(0);
    (0..max_len)
        .flat_map(|idx| buckets.iter().filter_map(move |b| b.get(idx).cloned()))
        .enumerate()
        .map(|(i, mut lvl)| {
            let id = (i + 1) as u32;
            lvl.id = id;
            lvl.name = format!("Level {}", id);
            lvl
        })
        .collect()
}

pub struct LevelRepository {
    packs: HashMap<PackKey, Vec<LevelData>>,
}

impl LevelRepository {
    pub fn load_embedded() -> Result<Self, serde_json::Error> {
        let mut packs = HashMap::new();

        // 6x6
        let p6_d4: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d4.json"))?;
        let p6_d5: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d5.json"))?;
        let p6_d6: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d6.json"))?;
        let p6_d7: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d7.json"))?;
        let p6_d8: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d8.json"))?;
        let p6_d9: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d9.json"))?;
        let p6_d10: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d10.json"))?;
        let p6_d11: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d11.json"))?;
        let p6_d12: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d12.json"))?;
        let p6_d13: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_6_d13.json"))?;

        packs.insert(
            PackKey::new(FieldSize::Small6x6, DifficultyTier::Relaxed),
            mix_zigzag(&[p6_d4, p6_d5, p6_d6]),
        );
        packs.insert(
            PackKey::new(FieldSize::Small6x6, DifficultyTier::Challenging),
            mix_zigzag(&[p6_d7, p6_d8, p6_d9]),
        );
        packs.insert(
            PackKey::new(FieldSize::Small6x6, DifficultyTier::Hard),
            mix_zigzag(&[p6_d10, p6_d11, p6_d12, p6_d13]),
        );

        // 8x8
        let p8_d4: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d4.json"))?;
        let p8_d5: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d5.json"))?;
        let p8_d6: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d6.json"))?;
        let p8_d7: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d7.json"))?;
        let p8_d8: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d8.json"))?;
        let p8_d9: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d9.json"))?;
        let p8_d10: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d10.json"))?;
        let p8_d11: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d11.json"))?;
        let p8_d12: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d12.json"))?;
        let p8_d13: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_8_d13.json"))?;

        packs.insert(
            PackKey::new(FieldSize::Medium8x8, DifficultyTier::Relaxed),
            mix_zigzag(&[p8_d4, p8_d5, p8_d6]),
        );
        packs.insert(
            PackKey::new(FieldSize::Medium8x8, DifficultyTier::Challenging),
            mix_zigzag(&[p8_d7, p8_d8, p8_d9]),
        );
        packs.insert(
            PackKey::new(FieldSize::Medium8x8, DifficultyTier::Hard),
            mix_zigzag(&[p8_d10, p8_d11, p8_d12, p8_d13]),
        );

        // 10x10
        let p10_d4: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d4.json"))?;
        let p10_d5: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d5.json"))?;
        let p10_d6: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d6.json"))?;
        let p10_d7: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d7.json"))?;
        let p10_d8: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d8.json"))?;
        let p10_d9: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d9.json"))?;
        let p10_d10: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d10.json"))?;
        let p10_d11: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d11.json"))?;
        let p10_d12: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d12.json"))?;
        let p10_d13: Vec<LevelData> =
            serde_json::from_str(include_str!("../../assets/levels/pack_10_d13.json"))?;

        packs.insert(
            PackKey::new(FieldSize::Big10x10, DifficultyTier::Relaxed),
            mix_zigzag(&[p10_d4, p10_d5, p10_d6]),
        );
        packs.insert(
            PackKey::new(FieldSize::Big10x10, DifficultyTier::Challenging),
            mix_zigzag(&[p10_d7, p10_d8, p10_d9]),
        );
        packs.insert(
            PackKey::new(FieldSize::Big10x10, DifficultyTier::Hard),
            mix_zigzag(&[p10_d10, p10_d11, p10_d12, p10_d13]),
        );

        Ok(Self { packs })
    }

    pub fn get_pack(&self, key: PackKey) -> &[LevelData] {
        self.packs.get(&key).map_or(&[], Vec::as_slice)
    }

    pub fn get_level(&self, key: PackKey, index: usize) -> Option<&LevelData> {
        self.get_pack(key).get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix_zigzag() {
        let make_lvl = |id: u32, par: u32| LevelData {
            id,
            name: format!("Original {}", id),
            width: 6,
            height: 6,
            exit: ExitPosition {
                side: super::super::board::ExitSide::Right,
                row: 2,
                col: 0,
            },
            vehicles: vec![],
            obstacles: vec![],
            par_moves: par,
        };

        let bucket_a = vec![make_lvl(1, 4), make_lvl(2, 4), make_lvl(3, 4)];
        let bucket_b = vec![make_lvl(10, 5), make_lvl(20, 5)];
        let bucket_c = vec![make_lvl(100, 6)];

        let mixed = mix_zigzag(&[bucket_a, bucket_b, bucket_c]);

        // Expected order:
        // idx 0: A[0] (par 4), B[0] (par 5), C[0] (par 6)
        // idx 1: A[1] (par 4), B[1] (par 5)
        // idx 2: A[2] (par 4)
        assert_eq!(mixed.len(), 6);
        let pars: Vec<u32> = mixed.iter().map(|l| l.par_moves).collect();
        assert_eq!(pars, vec![4, 5, 6, 4, 5, 4]);

        let ids: Vec<u32> = mixed.iter().map(|l| l.id).collect();
        assert_eq!(ids, vec![1, 2, 3, 4, 5, 6]);

        let names: Vec<&str> = mixed.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["Level 1", "Level 2", "Level 3", "Level 4", "Level 5", "Level 6"]
        );
    }

    #[test]
    fn test_embedded_packs_load() {
        let repo = LevelRepository::load_embedded().expect("Embedded levels should load");
        for size in FieldSize::ALL {
            let relaxed = repo.get_pack(PackKey::new(size, DifficultyTier::Relaxed));
            assert!(
                !relaxed.is_empty(),
                "Relaxed pack for {:?} should not be empty",
                size
            );
        }
    }
}
