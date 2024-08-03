//! Module containing specific bitboards


use pyo3::prelude::*;


#[pyclass(module = "ChessProject", get_all, set_all)]
#[derive(Clone, Debug)]
pub struct SpecialBitBoards {
    // specific bitboard masks
    pub file_ab: u64,
    pub file_gh: u64,
    centre: u64,
    extended_centre: u64,
    king_side: u64,
    queen_side: u64,
    pub king_span_c7: u64, // where c7 king can attack
    pub knight_span_c6: u64, // where c6 knight can attack
    pub not_allied_pieces: u64, // if in white func: all pieces white can capture (not black king
    pub enemy_pieces: u64, // if in white func: black pieces but no black king
    pub empty: u64,
    pub occupied: u64,

    // region based bitboard masks
    pub rank_masks: [u64; 8], // from rank 8 to rank 1
    pub file_masks: [u64; 8], // from file a to file h
    pub diagonal_masks: [u64; 15], // from top left to bottom right
    pub anti_diagonal_masks: [u64; 15], // from top right to bottom left
}


#[pymethods]
impl SpecialBitBoards {
    #[new]
    pub fn new() -> Self {
        SpecialBitBoards {
            file_ab: 13889313184910721216,
            file_gh: 217020518514230019,
            centre: 103481868288,
            extended_centre: 66229406269440,
            king_side: 1085102592571150095,
            queen_side: 17361641481138401520,
            king_span_c7: 8093091675687092224,
            knight_span_c6: 5802888705324613632,
            not_allied_pieces: 0,
            enemy_pieces: 0,
            empty: 0,
            occupied: 0,
            rank_masks: [
                18374686479671623680,
                71776119061217280,
                280375465082880,
                1095216660480,
                4278190080,
                16711680,
                65280,
                255,
            ],
            file_masks: [
                9259542123273814144,
                4629771061636907072,
                2314885530818453536,
                1157442765409226768,
                578721382704613384,
                289360691352306692,
                144680345676153346,
                72340172838076673,
            ],
            diagonal_masks: [
                9223372036854775808,
                4647714815446351872,
                2323998145211531264,
                1161999622361579520,
                580999813328273408,
                290499906672525312,
                145249953336295424,
                72624976668147840,
                283691315109952,
                1108169199648,
                4328785936,
                16909320,
                66052,
                258,
                1,
            ],
            anti_diagonal_masks: [
                72057594037927936,
                144396663052566528,
                288794425616760832,
                577588855528488960,
                1155177711073755136,
                2310355422147575808,
                4620710844295151872,
                9241421688590303745,
                36099303471055874,
                141012904183812,
                550831656968,
                2151686160,
                8405024,
                32832,
                128,
            ],
        }
    }
}