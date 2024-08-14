//! Module holding transposition table information


use pyo3::prelude::*;


pub enum HashFlag {
    Exact,
    Alpha,
    Beta,
}


#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone, Debug)]
pub struct TransTableEntry {
    pub hash_key: u64,
    pub depth: u32, // current search depth
    pub flag: i32, // flag the type of move: fail-low, fail-high, PV
    pub score: i32, // alpha, beta, or PV
}


#[pymethods]
impl TransTableEntry {
    #[new]
    pub fn new() -> Self {
        TransTableEntry {
            hash_key: 0,
            depth: 0,
            flag: 0,
            score: 0,
        }
    }
}


#[pyclass(module = "ChessProject", get_all, set_all)]
pub struct TransTable {
    pub table: Vec<TransTableEntry>, // large size so heap allocated
}


#[pymethods]
impl TransTable {
    /*
    Table Memory Analysis:

    TransTableEntry = u64 + u32 + i32 + i32 = 160 bits
    Vec = heap-allocated buffer pointer + length + capacity = 3 * u64 = 192 bits
    Table = TRANS_TABLE_SIZE * TransTableEntry + Vec = 5_000_000 * 160 + 192 ~= 100 MB
    */
    pub const TRANS_TABLE_SIZE: usize = 5_000_000;
    pub const NO_HASH_ENTRY: i32 = 100000;
    #[new]
    pub fn new() -> Self {
        TransTable {
            table: vec![TransTableEntry::new(); TransTable::TRANS_TABLE_SIZE],
        }
    }


    pub fn clearTable(&mut self) {
        for i in 0..TransTable::TRANS_TABLE_SIZE {
            self.table[i].hash_key = 0;
            self.table[i].depth = 0;
            self.table[i].flag = 0;
            self.table[i].score = 0;
        }
    }


    pub fn readEntry(&self, alpha: i32, beta: i32, hash_key: u64, depth: i32, ply: u32) -> i32 {
        let table_entry: &TransTableEntry = &self.table[hash_key as usize % TransTable::TRANS_TABLE_SIZE];
        if table_entry.hash_key == hash_key && table_entry.depth as i32 >= depth {
            let mut score: i32 = table_entry.score;
            // add distance from root to current node if mate score
            score += if score > 48000 {-(ply as i32)} else if score < -48000 {ply as i32} else {0};
            if table_entry.flag == HashFlag::Exact as i32 {
                return score;
            }
            if table_entry.flag == HashFlag::Alpha as i32 && score <= alpha {
                return alpha;
            }
            if table_entry.flag == HashFlag::Beta as i32 && score >= beta {
                return beta;
            }
        }
        TransTable::NO_HASH_ENTRY
    }


    pub fn writeEntry(&mut self, mut score: i32, hash_key: u64, depth: u32, ply: u32, hash_flag: i32) {
        let table_entry: &mut TransTableEntry = &mut self.table[hash_key as usize % TransTable::TRANS_TABLE_SIZE];
        // mate scores should be path independant in table, remove distance from root to current node from score
        score += if score > 48000 {ply as i32} else if score < -48000 {-(ply as i32)} else {0};
        table_entry.hash_key = hash_key;
        table_entry.depth = depth;
        table_entry.flag = hash_flag;
        table_entry.score = score;
    }
}
