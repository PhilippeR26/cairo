// TODO(yg): decide what's the right name.

use super::try_from_felt252::{TryFromFelt252, TryFromFelt252Libfunc};
use crate::define_libfunc_hierarchy;
use crate::extensions::lib_func::{
    DeferredOutputKind, LibfuncSignature, OutputVarInfo, SierraApChange,
    SignatureSpecializationContext,
};
use crate::extensions::{
    NamedType, NoGenericArgsGenericLibfunc, NoGenericArgsGenericType, OutputVarReferenceInfo,
    SpecializationError,
};
use crate::ids::GenericTypeId;

/// Type for u248.
#[derive(Default)]
pub struct U248Type {}
impl NoGenericArgsGenericType for U248Type {
    const ID: GenericTypeId = GenericTypeId::new_inline("u248");
    const STORABLE: bool = true;
    const DUPLICATABLE: bool = true;
    const DROPPABLE: bool = true;
    const ZERO_SIZED: bool = false;
}

// TODO(yg): what libfuncs do we really need?
define_libfunc_hierarchy! {
    pub enum U248Libfunc {
        TryFromFelt252(U248FromFelt252Libfunc),
        // TODO(yg): add the rest?
        // ToFelt252(U248ToFelt252Libfunc),
        // Add(U248AddLibfunc),
        // ShiftLeft(U248ShiftLeftLibfunc),
        // ShiftRight(U248ShiftRightLibfunc),
        // And(U248AndLibfunc),
        // Xor(U248XorLibfunc),
        // Or(BoolOrLibfunc),
    }, U248ConcreteLibfunc
}

/// Libfunc for attempting to convert a felt252 into a u248.
#[derive(Default)]
pub struct U248FromFelt252Trait;
impl TryFromFelt252 for U248FromFelt252Trait {
    const STR_ID: &'static str = "u248_try_from_felt252";
    const GENERIC_TYPE_ID: GenericTypeId = <U248Type as NoGenericArgsGenericType>::ID;
}

type U248FromFelt252Libfunc = TryFromFelt252Libfunc<U248FromFelt252Trait>;
