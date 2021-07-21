pub fn memory_representation() {
    // Stack and Heap:
    // Data which has fixed and known size at compile time is stored on the STACK otherwise it is
    // stored on the HEAP
    
    // 1: Types that hold data entirely stored on the STACK implement Copy trait: meaning the data
    //    of such types can be copied to produce a new identical value.
    // 2: Types that hold(rather refer to) data stored on the HEAP dont implement Copy trait:
    //    meaning the data of such types can not be copied to produce a new identical value. This
    //    is because when stack data(i.e. the reference to the actual data) of such types get copied,
    //    the heap data is still not copied which is a heavy process. Such types are moved when
    //    attempting to assign it to a new variable. Move means that the reference is copied and
    //    the older variable is made invalid

    let a  = 10; // Implements Copy trait because this data is stored on the STACK
    let b = a; // since `a` implements Copy trait, the data that `a` is holding(on the Stack) is
    // copied and `b` gets a copy of `10`. Both `a` and `b` are valid and usable
    
    let s = String::from("hello world"); // DOES NOT implement Copy trait because the actual data is
    // stored on the HEAP, only the reference to that data is stored on the STACK
    let ss = s; // since `s` DOES NOT implement Copy trait, `ss` gets a copy of `s` but only the 
    // stack part and `s` is made invalid afterwards
    // println!("{}", s); // this is an error because attempting to use an invalid variable
}

