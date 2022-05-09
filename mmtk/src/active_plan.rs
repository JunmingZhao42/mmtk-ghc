use mmtk::Plan;
use mmtk::vm::ActivePlan;
use mmtk::util::opaque_pointer::*;
use mmtk::Mutator;
use crate::GHCVM;
use crate::SINGLETON;

pub struct VMActivePlan<> {}

impl ActivePlan<GHCVM> for VMActivePlan {
    fn global() -> &'static dyn Plan<VM=GHCVM> {
        SINGLETON.get_plan()
    }

    fn number_of_mutators() -> usize {
        unimplemented!()
    }

    fn is_mutator(_tls: VMThread) -> bool {
        // FIXME
        true
    }

    fn mutator(_tls: VMMutatorThread) -> &'static mut Mutator<GHCVM> {
        unimplemented!()
    }

    fn reset_mutator_iterator() {
        unimplemented!()
    }

    fn get_next_mutator() -> Option<&'static mut Mutator<GHCVM>> {
        unimplemented!()
    }
}
