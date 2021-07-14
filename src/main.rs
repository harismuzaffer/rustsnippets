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
pub mod traits_and_generics;
pub mod lifetimes;
pub mod closures_iterators;
pub mod expressions;


fn main() {
    // let c = closures_iterators::closures::Cacher::new(|a| a);
    // let cc = closures_iterators::closures::Cacher {
    //     calculation: |a| a,
    // };

    closures_iterators::iterators::consuming_iterators::filter_with_map();
}

