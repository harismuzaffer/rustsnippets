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
pub mod stack_vs_heap;


fn main() {
    // let c = closures_iterators::closures::Cacher::new(|a| a);
    // let cc = closures_iterators::closures::Cacher {
    //     calculation: |a| a,
    // };

    // closures_iterators::iterators::consuming_iterators::filter_with_map();
    // let v1 = vec![1, 2, 3];

    // let mut v1_iter = v1.iter();

    // let g = String::from("ge");

    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));
    // println!("next item is {}", v1_iter.next().expect("ohh none"));    
    // more_lifetimes::apply_more_lifetimes_v2();

    let v: Vec<i32> = vec![1, 12, 7];
    let vv = v.iter();
    let xx: i32 = 8;
    // let vvv: Vec<&i32> = vv.filter(|x| **x > xx).collect();
    let vvv = vv.filter(|x| **x > xx);
    println!("this is: {:?}", v);

    let v: Vec<i32> = vec![34, 22, 17];
    let mut vv = v.iter();
    let vv = vv.by_ref();
    // let vvv: Vec<i32> = vv.map(|x| x + 1).collect();
    let vvv: Vec<i32> = vv.map(|x| x + 100).collect();
    println!("this is: {:?}", vv);
    
    let v1 = vec![1, 2, 3];
    let mut it1 = v1.iter();
    let x = it1.any(|&x| x > 1);
    // println!("this is: {:?}", it1);

    let v2 = vec![1, 2, 3];
    let mut it2 = v2.iter();
    let it2 = it2.by_ref();
    let y: Vec<&i32> = it2.filter(|x| **x > 1).collect();
    println!("this is: {:?}", y);
    println!("this is: {:?}", it2);
    let mut v = vec![1, 2, 3, 4, 5, 6, 7];
    let vv = &mut v;
    let x = split_at_mut(vv, 33);
    println!("x is: {:?}", x);


    // Arc alows to have multiple owners of what is inside it i.e. Mutex of HashMap. But it wont
    // allow to mutate it. So it wont allow to mutate Mutex but what is inside of Mutex i.e.
    // HashMap can be mutated (by one thread at a time)
    // let a = Arc::new(Mutex::new(HashMap::new()));
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

