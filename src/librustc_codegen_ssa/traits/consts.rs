use super::MiscMethods;
use crate::mir::place::PlaceRef;
use rustc::mir::interpret::Allocation;
use rustc::mir::interpret::Scalar;
use rustc::ty::layout;
use rustc_target::spec::AddrSpaceIdx;

pub trait ConstMethods<'tcx>: MiscMethods<'tcx> {
    // Constant constructors
    fn const_null(&self, t: Self::Type) -> Self::Value;
    fn const_undef(&self, t: Self::Type) -> Self::Value;
    fn const_int(&self, t: Self::Type, i: i64) -> Self::Value;
    fn const_uint(&self, t: Self::Type, i: u64) -> Self::Value;
    fn const_uint_big(&self, t: Self::Type, u: u128) -> Self::Value;
    fn const_bool(&self, val: bool) -> Self::Value;
    fn const_i32(&self, i: i32) -> Self::Value;
    fn const_u32(&self, i: u32) -> Self::Value;
    fn const_u64(&self, i: u64) -> Self::Value;
    fn const_usize(&self, i: u64) -> Self::Value;
    fn const_u8(&self, i: u8) -> Self::Value;
    fn const_real(&self, t: Self::Type, val: f64) -> Self::Value;

    fn const_struct(&self, elts: &[Self::Value], packed: bool) -> Self::Value;

    fn const_to_uint(&self, v: Self::Value) -> u64;
    fn const_to_opt_u128(&self, v: Self::Value, sign_ext: bool) -> Option<u128>;

    fn const_as_cast(&self, v: Self::Value, space: AddrSpaceIdx) -> Self::Value;
    fn const_flat_as_cast(&self, v: Self::Value) -> Self::Value {
        self.const_as_cast(v, self.flat_addr_space())
    }

    fn is_const_integral(&self, v: Self::Value) -> bool;

    fn scalar_to_backend(
        &self,
        cv: Scalar,
        layout: &layout::Scalar,
        llty: Self::Type,
    ) -> Self::Value;
    fn from_const_alloc(
        &self,
        layout: layout::TyLayout<'tcx>,
        align: layout::Align,
        alloc: &Allocation,
        offset: layout::Size,
    ) -> PlaceRef<'tcx, Self::Value>;

    fn const_ptrcast(&self, val: Self::Value, ty: Self::Type) -> Self::Value;
}
