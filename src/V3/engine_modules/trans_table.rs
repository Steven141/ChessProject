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
    pub flag: i64, // flag the type of move: fail-low, fail-high, PV
    pub score: i64, // alpha, beta, or PV
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
    pub const TRANS_TABLE_SIZE: usize = 0x400000; // 4 MB
    pub const NO_HASH_ENTRY: i64 = 100000;
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


    pub fn readEntry(&self, alpha: i64, beta: i64, hash_key: u64, depth: u32) -> i64 {
        let table_entry: &TransTableEntry = &self.table[hash_key as usize % TransTable::TRANS_TABLE_SIZE];
        if table_entry.hash_key == hash_key && table_entry.depth >= depth {
            if table_entry.flag == HashFlag::Exact as i64 {
                return table_entry.score;
            }
            if table_entry.flag == HashFlag::Alpha as i64 && table_entry.score <= alpha {
                return alpha;
            }
            if table_entry.flag == HashFlag::Beta as i64 && table_entry.score >= beta {
                return beta;
            }
        }
        TransTable::NO_HASH_ENTRY
    }


    pub fn writeEntry(&mut self, score: i64, hash_key: u64, depth: u32, hash_flag: i64) {
        let table_entry: &mut TransTableEntry = &mut self.table[hash_key as usize % TransTable::TRANS_TABLE_SIZE];
        table_entry.hash_key = hash_key;
        table_entry.depth = depth;
        table_entry.flag = hash_flag;
        table_entry.score = score;
    }
}
