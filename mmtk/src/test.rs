use mmtk::vm::{EdgeVisitor};
use mmtk::util::{Address};
use super::stg_closures::*;
// use super::stg_info_table::*;
// use super::object_scanning::*;
use super::scanning::*;

use std::vec::*;

#[no_mangle]
pub unsafe extern "C" fn print_obj(obj : TaggedClosureRef){  
    let closure = Closure::from_ptr(obj.to_ptr());
    println!("obj in address {:?}:", obj.to_ptr());
    println!("{:?}", closure);

    // TODO: not working
    // match closure {
    //     Closure::Constr(_) => {
    //         println!("closure={:?}, {:?}", closure, 
    //                 StgConInfoTable::from_info_table(obj.get_info_table()).con_desc());
    //     }
    //     _ => {
    //         println!("{:?}", closure);
    //     }
    // }
}

struct CollectPointerVisitor {
    pub pointers : Vec<TaggedClosureRef>,   
}

impl EdgeVisitor for CollectPointerVisitor {
    fn visit_edge(&mut self, edge: Address) {
        self.pointers.push(TaggedClosureRef::from_address(edge));
    }
}

impl CollectPointerVisitor {
    fn new() -> Self {
        CollectPointerVisitor{pointers : Vec::new()}
    }
}


extern "C" {
    fn heap_view_closureSize(closure: *const StgClosure) -> usize;
    fn collect_pointers(closure: *const StgClosure, pointers: *mut *const StgClosure) -> usize;
}

#[no_mangle]
pub unsafe extern "C" fn rs_collect_pointers(obj : TaggedClosureRef) { 
    // keep a common set to iterate through all closures
    // recursively visit visitor.pointers
    // 1. set of obj to visit
    // 2. set of obj visited

    // Rust version of tracing all the pointers
    // println!("Start tracing pointers using Rust heap model...");
    let mut visited = Vec::new();
    let mut to_visit = Vec::new();
    to_visit.push(obj);

    let mut visitor = CollectPointerVisitor::new();
    while !to_visit.is_empty() {
        let x = to_visit.pop().expect("visitor empty but still poping element...");
        // println!("visiting this object {:?} in Rust", x.to_ptr());
        if !visited.contains(&x) {
            visit_closure(x, &mut visitor); // dereferencing the borrow?
            to_visit.append(&mut visitor.pointers); // this should clear visitor.pointers
            visited.push(x);
        }
    }

    println!();
    // C version of tracing all the pointers
    // println!("Start tracing pointers using C heap model...");
    let mut visited_c = Vec::new();
    let mut to_visit_c = Vec::new();
    to_visit_c.push(obj.to_ptr());
    
    while !to_visit_c.is_empty() {
        let mut x = to_visit_c.pop().expect("visitor empty but still poping element...");
        x = TaggedClosureRef::from_ptr(x as *mut StgClosure).to_ptr();
        if !visited_c.contains(&x) {
            // println!("visiting this object {:?} in C", x);
            let mut _to_visit : Vec<*const StgClosure> = Vec::with_capacity(heap_view_closureSize(x));
            let _n = collect_pointers(x, _to_visit.as_mut_ptr());

            _to_visit.set_len(_n); // update the length of the vector after visiting
            to_visit_c.append(&mut _to_visit); // this should clear _to_visit
            visited_c.push(x);
        }
    }

    // comparing
    // println!("\nFinish visiting all the pointers, comparing the two result...");

    assert_eq!(visited.len(), visited_c.len(), "Two vector not the same length");

    for (i, j) in visited.into_iter().zip(visited_c.into_iter()) {
        // print_obj(i);
        // print_obj(TaggedClosureRef::from_ptr(j as *mut StgClosure));
        // println!();
        assert_eq!(i.to_ptr(), TaggedClosureRef::from_ptr(j as *mut StgClosure).to_ptr(), 
        "Pointers not equal to each other {:?}, {:?}", i, j);
    }
}