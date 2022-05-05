use crate::DummyVM;
use mmtk::scheduler::*;
use mmtk::util::opaque_pointer::*;
use mmtk::vm::{EdgeVisitor, Scanning};
use mmtk::util::{Address, ObjectReference};
use mmtk::Mutator;

use super::types::*;
use super::stg_closures::*;
use super::stg_info_table::*;

pub struct VMScanning {}

impl Scanning<DummyVM> for VMScanning {
    fn scan_thread_roots<W: ProcessEdgesWork<VM = DummyVM>>() {
        unimplemented!()
    }
    fn scan_thread_root<W: ProcessEdgesWork<VM = DummyVM>>(
        _mutator: &'static mut Mutator<DummyVM>,
        _tls: VMWorkerThread,
    ) {
        unimplemented!()
    }
    fn scan_vm_specific_roots<W: ProcessEdgesWork<VM = DummyVM>>() {
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
            let closure: *const StgClosure = obj.to_address().to_ptr();
            let itbl: *const StgInfoTable = unsafe {(*closure).header.info_table.get_info_table()};
            match Closure::from_ptr(closure) {
                Closure::MVar(mvar) => {
                    ev.visit_edge(Address::from_ptr(mvar.head));
                    ev.visit_edge(Address::from_ptr(mvar.tail));
                    ev.visit_edge(Address::from_ptr(mvar.value.to_ptr()));
                }
                Closure::TVar(tvar) => {
                    ev.visit_edge(Address::from_ptr(tvar.current_value.to_ptr()));
                    ev.visit_edge(Address::from_ptr(tvar.first_watch_queue_entry));
                }
                // TODO: Check these two implementations ... 
                Closure::Thunk(thunk) => {
                    let end : u32 = (&*itbl).layout.payload.ptrs;
                    for n in 1..end {
                        let edge = thunk.payload.get(n as usize);
                        ev.visit_edge(Address::from_ptr(edge.to_ptr()));
                    }
                }
                Closure::Constr(closr) => {
                    let end : u32 = (&*itbl).layout.payload.ptrs;
                    for n in 1..end {
                        let edge = closr.payload.get(n as usize);
                        ev.visit_edge(Address::from_ptr(edge.to_ptr()));
                    }
                }
                Closure::Weak(weak) => {
                    ev.visit_edge(Address::from_ptr(weak.value.to_ptr()));
                    ev.visit_edge(Address::from_ptr(weak.key.to_ptr()));
                    ev.visit_edge(Address::from_ptr(weak.finalizer.to_ptr()));
                    ev.visit_edge(Address::from_ptr(weak.cfinalizers.to_ptr()));
                }
                Closure::BlockingQueue(bq) => {
                    ev.visit_edge(Address::from_ptr(bq.owner));
                    ev.visit_edge(Address::from_ptr(bq.queue));
                    ev.visit_edge(Address::from_ptr(bq.link));
                }
                Closure::ThunkSelector(selector) => {
                    ev.visit_edge(Address::from_ptr(selector.selectee.to_ptr()));
                }
                Closure::PausedEval(fun) => {
                    ev.visit_edge(Address::from_ptr(fun.fun.to_ptr()));
                    // TODO: scavenge stack
                }
                // TODO: PAP and AP using same struct?
                Closure::PartialAppliedFun(fun) => {
                    ev.visit_edge(Address::from_ptr(fun.fun.to_ptr()));
                    // TODO: scavenge_PAP_payload
                }
                Closure::AppliedFun(fun) => {
                    ev.visit_edge(Address::from_ptr(fun.fun.to_ptr()));
                    // TODO: scavenge_AP (just call scavenge_PAP_payload ?)
                }
                Closure::ArrBytes(_) => { return; }
                Closure::ArrMutPtr(_array) => {
                    // TODO: scavenge_mut_arr_ptrs
                }
                Closure::ArrMutPtrSmall(array) => {
                    let end = array.ptrs;
                    for n in 1..end {
                        let edge = array.payload.get(n);
                        ev.visit_edge(Address::from_ptr(edge.to_ptr()));
                    }
                }
                Closure::TSO(tso) => {
                    ev.visit_edge(Address::from_ptr(tso.blocked_exceptions));
                    ev.visit_edge(Address::from_ptr(tso.blocking_queue));
                    ev.visit_edge(Address::from_ptr(tso.trec));
                    ev.visit_edge(Address::from_ptr(tso.stackobj));
                    ev.visit_edge(Address::from_ptr(tso.link));

                    if tso.why_blocked == StgTSOBlocked::BLOCKED_ON_MVAR
                    || tso.why_blocked == StgTSOBlocked::BLOCKED_ON_MVAR_READ
                    || tso.why_blocked == StgTSOBlocked::BLOCKED_ON_BLACK_HOLE
                    || tso.why_blocked == StgTSOBlocked::BLOCKED_ON_MSG_THROW_TO
                    || tso.why_blocked == StgTSOBlocked::NOT_BLOCKED {
                        // TODO: implement struct for blocked_info.closure
                        // ev.visit_edge(Address::from_ptr(tso.block_info.closure));
                    }

                    ev.visit_edge(Address::from_ptr(tso.tso_link_prev));
                    ev.visit_edge(Address::from_ptr(tso.tso_link_next));
                }
                Closure::Stack(_stack) => {
                    // TODO: scanvenge stack
                }
                // TODO: check this
                Closure::TRecChunk(trec_chunk) => {
                    ev.visit_edge(Address::from_ptr(trec_chunk.prev_chunk));

                    let end = trec_chunk.next_entry_idx;
                    for n in 1..end {
                        let trec_entry = &trec_chunk.entries[n];
                        ev.visit_edge(Address::from_ptr(trec_entry.tvar));
                        ev.visit_edge(Address::from_ptr(trec_entry.expected_value.to_ptr()));
                        ev.visit_edge(Address::from_ptr(trec_entry.new_value.to_ptr()));
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