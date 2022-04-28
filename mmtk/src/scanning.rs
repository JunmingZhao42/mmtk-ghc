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
        unsafe{
            /*
            let closure: *const StgClosure = obj.to_address().to_ptr();
            let itbl: *const StgInfoTable = unsafe {(*closure).header.info_table.get_info_table()};
            
            
            match (*itbl).type_ {
                StgClosureType::MVAR_CLEAN | StgClosureType::MVAR_DIRTY => {
                    let mvar: *const StgMVar = closure.cast();
                    ev.visit_edge(Address::from_ptr((*mvar).head));
                    ev.visit_edge(Address::from_ptr((*mvar).tail));
                    ev.visit_edge(Address::from_ptr((*mvar).value));
                },
                StgClosureType::TVAR => {

                },
                // et cetera
            }
            */
            
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