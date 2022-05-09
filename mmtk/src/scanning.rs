use crate::GHCVM;
use mmtk::scheduler::*;
use mmtk::util::opaque_pointer::*;
use mmtk::vm::{EdgeVisitor, Scanning};
use mmtk::util::{Address, ObjectReference};
use mmtk::Mutator;

use super::stg_closures::*;
use super::stg_info_table::*;
use super::object_scanning::*;


pub struct VMScanning {}

impl Scanning<GHCVM> for VMScanning {
    fn scan_thread_roots<W: ProcessEdgesWork<VM = GHCVM>>() {
        unimplemented!()
    }
    fn scan_thread_root<W: ProcessEdgesWork<VM = GHCVM>>(
        _mutator: &'static mut Mutator<GHCVM>,
        _tls: VMWorkerThread,
    ) {
        unimplemented!()
    }
    fn scan_vm_specific_roots<W: ProcessEdgesWork<VM = GHCVM>>() {
        unimplemented!()
    }

    /// Delegated scanning of a object, visiting each pointer field
    /// encountered.
    ///
    /// Arguments:
    /// * `tls`: The VM-specific thread-local storage for the current worker.
    /// * `object`: The object to be scanned.
    /// * `edge_visitor`: Called back for each edge.
    fn scan_object<EV: EdgeVisitor>(
        _tls: VMWorkerThread,
        obj: ObjectReference,
        ev: &mut EV,
    ) {
        unsafe {
            let closure_ref = TaggedClosureRef::from_object_reference(obj);            
            let itbl: &'static StgInfoTable = closure_ref.get_info_table();
            match closure_ref.to_closure() {
                Closure::MVar(mvar) => {
                    ev.visit_edge(Address::from_ptr(mvar.head));
                    ev.visit_edge(Address::from_ptr(mvar.tail));
                    ev.visit_edge(mvar.value.to_address());
                }
                Closure::TVar(tvar) => {
                    ev.visit_edge(tvar.current_value.to_address());
                    ev.visit_edge(Address::from_ptr(tvar.first_watch_queue_entry));
                }
                Closure::Thunk(thunk) => {
                    let n_ptrs : u32 = (&*itbl).layout.payload.ptrs;
                    for n in 0..n_ptrs {
                        let edge = thunk.payload.get(n as usize);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::Constr(closr) => {
                    let n_ptrs : u32 = (&*itbl).layout.payload.ptrs;
                    for n in 0..n_ptrs {
                        let edge = closr.payload.get(n as usize);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::Weak(weak) => {
                    ev.visit_edge(weak.value.to_address());
                    ev.visit_edge(weak.key.to_address());
                    ev.visit_edge(weak.finalizer.to_address());
                    ev.visit_edge(weak.cfinalizers.to_address());
                }
                Closure::BlockingQueue(bq) => {
                    ev.visit_edge(Address::from_ptr(bq.owner));
                    ev.visit_edge(Address::from_ptr(bq.queue));
                    ev.visit_edge(Address::from_ptr(bq.link));
                }
                Closure::ThunkSelector(selector) => {
                    ev.visit_edge(selector.selectee.to_address());
                }
                Closure::ApStack(fun) => {
                    ev.visit_edge(fun.fun.to_address());
                    scan_stack(_tls, fun.iter(), ev);
                }
                // TODO: FUN
                // TODO: PAP and AP using same struct?
                Closure::PartialAppliedFun(fun) => {
                    ev.visit_edge(fun.fun.to_address());
                    // TODO: scavenge_PAP_payload
                }
                Closure::AppliedFun(fun) => {
                    ev.visit_edge(fun.fun.to_address());
                    // TODO: scavenge_AP (just call scavenge_PAP_payload ?)
                }
                Closure::ArrBytes(_) => { return; }
                Closure::ArrMutPtr(array) => {
                    // TODO: scavenge_mut_arr_ptrs
                    // use scavenge_mut_arr_ptrs
                    // TODO: use the optimised version later for card marking
                    // should mark the bits : mark everything to 0 
                    // (bool: whether there's an inter generation pointer (old to young))
                    let n_ptrs = array.ptrs;
                    for n in 0..n_ptrs {
                        let edge = array.payload.get(n as usize);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::ArrMutPtrSmall(array) => {
                    let n_ptrs = array.ptrs;
                    for n in 0..n_ptrs {
                        let edge = array.payload.get(n);
                        ev.visit_edge(edge.to_address());
                    }
                }
                Closure::TSO(tso) => {
                    scan_TSO(_tls, tso, ev);
                }
                Closure::Stack(stack) => {
                    scan_stack(_tls, stack.iter(), ev);
                }
                Closure::TRecChunk(trec_chunk) => {
                    ev.visit_edge(Address::from_ptr(trec_chunk.prev_chunk));

                    let n_ptrs = trec_chunk.next_entry_idx;
                    for n in 0..n_ptrs {
                        let trec_entry = &trec_chunk.entries[n];
                        ev.visit_edge(Address::from_ptr(trec_entry.tvar));
                        ev.visit_edge(trec_entry.expected_value.to_address());
                        ev.visit_edge(trec_entry.new_value.to_address());
                    }
                }
                Closure::Indirect(ind) => {
                    ev.visit_edge(Address::from_ptr(ind.indirectee.to_ptr()));
                }
                Closure::IndirectStatic(ind) => {
                    ev.visit_edge(Address::from_ptr(ind.indirectee.to_ptr()));
                }
                // TODO: scavenge_compact for COMPACT_NFDATA?
                _ => panic!("scavenge_one: strange object type={:?}, address={:?}", 
                            (&*itbl).type_, itbl)                
            }
            
            
        }

    }


    fn notify_initial_thread_scan_complete(_partial_scan: bool, _tls: VMWorkerThread) {
        unimplemented!()
    }
    fn supports_return_barrier() -> bool {
        unimplemented!()
    }
    fn prepare_for_roots_re_scanning() {
        unimplemented!()
    }
}