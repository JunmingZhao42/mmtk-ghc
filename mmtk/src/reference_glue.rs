use mmtk::vm::ReferenceGlue;
use mmtk::util::ObjectReference;
use mmtk::TraceLocal;
use mmtk::util::opaque_pointer::*;
use crate::GHCVM;

pub struct VMReferenceGlue {}

impl ReferenceGlue<GHCVM> for VMReferenceGlue {
    fn set_referent(_reference: ObjectReference, _referent: ObjectReference) {
        unimplemented!()
    }
    fn get_referent(_object: ObjectReference) -> ObjectReference {
        unimplemented!()
    }
    fn process_reference<T: TraceLocal>(_trace: &mut T, _reference: ObjectReference, _tls: VMWorkerThread) -> ObjectReference {
        unimplemented!()
    }
}
