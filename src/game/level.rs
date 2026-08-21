use super::board::{Board, ExitPosition};
use super::vehicle::Vehicle;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackType {
    Grid6x6,
    Grid8x8,
    Grid10x10,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub id: u32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub exit: ExitPosition,
    pub vehicles: Vec<Vehicle>,
    #[serde(default = "default_par")]
    pub par_moves: u32,
}

const fn default_par() -> u32 {
    10
}

impl LevelData {
    pub fn to_board(&self) -> Board {
        Board::new(self.width, self.height, self.exit, self.vehicles.clone())
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

pub struct LevelRepository {
    pub pack_6x6: Vec<LevelData>,
    pub pack_8x8: Vec<LevelData>,
    pub pack_10x10: Vec<LevelData>,
}

impl LevelRepository {
    pub fn load_embedded() -> Result<Self, serde_json::Error> {
        let p6_str = include_str!("../../assets/levels/pack_6x6.json");
        let p8_str = include_str!("../../assets/levels/pack_8x8.json");
        let p10_str = include_str!("../../assets/levels/pack_10x10.json");

        let pack_6x6: Vec<LevelData> = serde_json::from_str(p6_str)?;
        let pack_8x8: Vec<LevelData> = serde_json::from_str(p8_str)?;
        let pack_10x10: Vec<LevelData> = serde_json::from_str(p10_str)?;

        Ok(Self {
            pack_6x6,
            pack_8x8,
            pack_10x10,
        })
    }

    pub fn get_pack(&self, pack: PackType) -> &[LevelData] {
        match pack {
            PackType::Grid6x6 => &self.pack_6x6,
            PackType::Grid8x8 => &self.pack_8x8,
            PackType::Grid10x10 => &self.pack_10x10,
        }
    }

    pub fn get_level(&self, pack: PackType, index: usize) -> Option<&LevelData> {
        self.get_pack(pack).get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::solver::solve;

    #[test]
    fn test_all_embedded_levels_are_solvable() {
        let repo = LevelRepository::load_embedded().expect("Failed to parse embedded levels");
        assert!(!repo.pack_6x6.is_empty());
        assert!(!repo.pack_8x8.is_empty());
        assert!(!repo.pack_10x10.is_empty());

        for (pack_type, pack) in [
            (PackType::Grid6x6, &repo.pack_6x6),
            (PackType::Grid8x8, &repo.pack_8x8),
            (PackType::Grid10x10, &repo.pack_10x10),
        ] {
            for lvl in pack {
                let solution = solve(lvl.width, lvl.height, lvl.exit, &lvl.vehicles);
                assert!(
                    solution.is_some(),
                    "Level {} ({:?}) should be solvable",
                    lvl.name,
                    pack_type
                );
                let moves = solution.unwrap();
                assert_eq!(
                    moves, lvl.par_moves,
                    "Par moves for level {} should match BFS optimal moves",
                    lvl.name
                );
            }
        }
    }
}
