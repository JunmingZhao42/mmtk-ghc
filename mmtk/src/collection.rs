use mmtk::vm::Collection;
use mmtk::vm::GCThreadContext;
use mmtk::MutatorContext;
use mmtk::util::opaque_pointer::*;
use mmtk::scheduler::*;
use crate::GHCVM;

pub struct VMCollection {}

impl Collection<GHCVM> for VMCollection {
    fn stop_all_mutators<E: ProcessEdgesWork<VM=GHCVM>>(_tls: VMWorkerThread) {
        unimplemented!()
    }

    fn resume_mutators(_tls: VMWorkerThread) {
        unimplemented!()
    }

    fn block_for_gc(_tls: VMMutatorThread) {
        panic!("block_for_gc is not implemented")
    }

    fn spawn_gc_thread(_tls: VMThread, _ctx: GCThreadContext<GHCVM>) {

    }

    fn prepare_mutator<T: MutatorContext<GHCVM>>(_tls_w: VMWorkerThread, _tls_m: VMMutatorThread, _mutator: &T) {
        unimplemented!()
    }
}
