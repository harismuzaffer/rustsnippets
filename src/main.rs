pub mod strings;
pub mod datatypes;
pub mod structs;
pub mod enums;
pub mod control_flow;
pub mod vectors;
pub mod guessing_game;
pub mod functions;
pub mod safety_rules;
pub mod hash_maps;
pub mod string_collection;
pub mod error_handling;


fn main() {
    // error_handling::unrecoveralble::index_out_of_bound();
    // error_handling::recoveralble::file_not_found();
    // error_handling::recoveralble::file_not_found_error_kind();
    error_handling::recoveralble::unwrap_expect();
}

