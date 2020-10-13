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

fn main() {
    // guessing_game::play();
    // datatypes::tuples::tupletest();
    // control_flow::test_for_equality(56, 56);
    // control_flow::loop_n_times(10);
    // functions::fibonacci(1290);
    // safety_rules::owning::owning();
    // strings::slice_string();
    // safety_rules::references::mutable_and_immutable_references();
    // vectors::get_second_itme_from_vector();
    // vectors::vectors_test();
    // enums::enums_with_struct();
    // functions::functions_pass_and_return::test_pass_and_return();
    // string_collection::string_manipulation();
    // string_collection::iterate_string();
    // string_collection::concatenate_strings();

    // hash_maps::hash_map_using_collect();
    hash_maps::manipulating_hash_maps();
    hash_maps::update_hashmap();
}
