use std::collections::HashMap;

use rsdd::{
    builder::{bdd::RobddBuilder, cache::AllIteTable, BottomUpBuilder},
    constants::primes,
    repr::{BddPtr, Cnf, DDNNFPtr, PartialModel, VarLabel, VarOrder, WmcParams},
    util::semirings::{ExpectedUtility, FiniteField, RealSemiring, Semiring},
};

#[ocaml::sig]
pub struct RsddBddPtr(BddPtr<'static>);
ocaml::custom!(RsddBddPtr);

#[ocaml::sig]
pub struct RsddBddBuilder(RobddBuilder<'static, AllIteTable<BddPtr<'static>>>);
ocaml::custom!(RsddBddBuilder);

#[ocaml::sig]
pub struct RsddCnf(Cnf);
ocaml::custom!(RsddCnf);

#[ocaml::sig]
pub struct RsddPartialModel(PartialModel);
ocaml::custom!(RsddPartialModel);

#[ocaml::sig]
pub struct RsddVarLabel(VarLabel);
ocaml::custom!(RsddVarLabel);

unsafe impl ocaml::ToValue for RsddVarLabel {
    fn to_value(&self, _rt: &ocaml::Runtime) -> ocaml::Value {
        unsafe { ocaml::Value::int64(self.0.value() as i64) }
    }
}

unsafe impl ocaml::FromValue for RsddVarLabel {
    fn from_value(v: ocaml::Value) -> Self {
        let i = unsafe { v.int64_val() };
        RsddVarLabel(VarLabel::new(i as u64))
    }
}

// disc/dice interface

#[ocaml::func]
#[ocaml::sig("int64 -> rsdd_bdd_builder")]
pub fn mk_bdd_builder_default_order(num_vars: u64) -> ocaml::Pointer<RsddBddBuilder> {
    RsddBddBuilder(RobddBuilder::<AllIteTable<BddPtr>>::new(
        VarOrder::linear_order(num_vars as usize),
    ))
    .into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> bool -> (int64 * rsdd_bdd_ptr)")]
pub fn bdd_new_var(builder: &'static RsddBddBuilder, polarity: bool) -> (u64, ocaml::Pointer<RsddBddPtr>) {
    let (lbl, ptr) = builder.0.new_var(polarity);
    (lbl.value(), RsddBddPtr(ptr).into())
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr")]
pub fn bdd_ite(
    builder: &'static RsddBddBuilder,
    if_var: &RsddBddPtr,
    then_var: &RsddBddPtr,
    else_var: &RsddBddPtr,
) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.ite(if_var.0, then_var.0, else_var.0)).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr")]
pub fn bdd_and(
    builder: &'static RsddBddBuilder,
    a: &RsddBddPtr,
    b: &RsddBddPtr,
) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.and(a.0, b.0)).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> rsdd_bdd_ptr")]
pub fn bdd_or(
    builder: &'static RsddBddBuilder,
    a: &RsddBddPtr,
    b: &RsddBddPtr,
) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.or(a.0, b.0)).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr")]
pub fn bdd_negate(
    builder: &'static RsddBddBuilder,
    bdd: &RsddBddPtr,
) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.negate(bdd.0)).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr")]
pub fn bdd_true(builder: &'static RsddBddBuilder) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.true_ptr()).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr")]
pub fn bdd_false(builder: &'static RsddBddBuilder) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.false_ptr()).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> bool")]
pub fn bdd_is_true(bdd: &RsddBddPtr) -> bool {
    bdd.0.is_true()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> bool")]
pub fn bdd_is_false(bdd: &RsddBddPtr) -> bool {
    bdd.0.is_false()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> bool")]
pub fn bdd_is_const(bdd: &RsddBddPtr) -> bool {
    bdd.0.is_const()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr -> rsdd_bdd_ptr -> bool")]
pub fn bdd_eq(builder: &'static RsddBddBuilder, a: &RsddBddPtr, b: &RsddBddPtr) -> bool {
    builder.0.eq(a.0, b.0)
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> int64")]
pub fn bdd_topvar(bdd: &RsddBddPtr) -> u64 {
    match (bdd.0).var_safe() {
        Some(x) => x.value(),
        None => 0, // TODO: provide a better version for this, maybe a Maybe/Option?
    }
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> rsdd_bdd_ptr")]
pub fn bdd_low(bdd: &RsddBddPtr) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(bdd.0.low()).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> rsdd_bdd_ptr")]
pub fn bdd_high(bdd: &RsddBddPtr) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(bdd.0.high()).into()
}

// real semiring

#[ocaml::sig]
pub struct RsddWmcParamsR(WmcParams<RealSemiring>);
ocaml::custom!(RsddWmcParamsR);

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> rsdd_wmc_params_r -> float")]
pub fn bdd_wmc(
    bdd: &RsddBddPtr,
    wmc: &RsddWmcParamsR,
) -> f64 {
    DDNNFPtr::unsmoothed_wmc(&bdd.0, &wmc.0).0
}

#[ocaml::func]
#[ocaml::sig("(float * float) list -> rsdd_wmc_params_r")]
pub fn new_wmc_params_r(weights: ocaml::List<(f64, f64)>) -> ocaml::Pointer<RsddWmcParamsR> {
    RsddWmcParamsR(WmcParams::new(HashMap::from_iter(
        weights
            .into_linked_list()
            .iter()
            .enumerate()
            .map(|(index, (a, b))| {
                (
                    VarLabel::new(index as u64),
                    (RealSemiring(*a), RealSemiring(*b)),
                )
            }),
    )))
    .into()
}

// branch & bound, expected semiring items
#[ocaml::sig]
pub struct RsddExpectedUtility(ExpectedUtility);
ocaml::custom!(RsddExpectedUtility);

#[ocaml::sig]
pub struct RsddWmcParamsEU(WmcParams<ExpectedUtility>);
ocaml::custom!(RsddWmcParamsEU);

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> rsdd_var_label list -> int64 -> rsdd_wmc_params_e_u -> rsdd_expected_utility * rsdd_partial_model")]
pub fn bdd_bb(
    bdd: &'static RsddBddPtr,
    join_vars: ocaml::List<RsddVarLabel>,
    num_vars: u64,
    wmc: &RsddWmcParamsEU,
) -> (
    ocaml::Pointer<RsddExpectedUtility>,
    ocaml::Pointer<RsddPartialModel>,
) {
    let (eu, pm) = bdd.0.bb(
        &join_vars
            .into_linked_list()
            .iter()
            .map(|x| x.0)
            .collect::<Vec<_>>(),
        num_vars as usize,
        &wmc.0,
    );
    (RsddExpectedUtility(eu).into(), RsddPartialModel(pm).into())
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_ptr -> rsdd_var_label list -> int64 -> rsdd_wmc_params_e_u -> rsdd_expected_utility * rsdd_partial_model")]
pub fn bdd_meu(
    bdd: &'static RsddBddPtr,
    decision_vars: ocaml::List<RsddVarLabel>,
    num_vars: u64,
    wmc: &RsddWmcParamsEU,
) -> (
    ocaml::Pointer<RsddExpectedUtility>,
    ocaml::Pointer<RsddPartialModel>,
) {
    let (eu, pm) = bdd.0.bb(
        &decision_vars
            .into_linked_list()
            .iter()
            .map(|x| x.0)
            .collect::<Vec<_>>(),
        num_vars as usize,
        &wmc.0,
    );
    (RsddExpectedUtility(eu).into(), RsddPartialModel(pm).into())
}

#[ocaml::func]
#[ocaml::sig("((float * float) * (float * float)) list -> rsdd_wmc_params_e_u")]
pub fn new_wmc_params_eu(
    weights: ocaml::List<((f64, f64), (f64, f64))>,
) -> ocaml::Pointer<RsddWmcParamsEU> {
    RsddWmcParamsEU(WmcParams::new(HashMap::from_iter(
        weights
            .into_linked_list()
            .iter()
            .enumerate()
            .map(|(index, (a, b))| {
                (
                    VarLabel::new(index as u64),
                    (ExpectedUtility(a.0, a.1), ExpectedUtility(b.0, b.1)),
                )
            }),
    )))
    .into()
}

// functions to help with user testing, but not used by disc/dice

#[ocaml::func]
#[ocaml::sig("string -> rsdd_cnf")]
pub fn cnf_from_dimacs(dimacs: &str) -> ocaml::Pointer<RsddCnf> {
    RsddCnf(Cnf::from_dimacs(dimacs)).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_cnf -> rsdd_bdd_ptr")]
pub fn bdd_builder_compile_cnf(
    builder: &'static RsddBddBuilder,
    cnf: &RsddCnf,
) -> ocaml::Pointer<RsddBddPtr> {
    RsddBddPtr(builder.0.compile_cnf(&cnf.0)).into()
}

#[ocaml::func]
#[ocaml::sig("rsdd_bdd_builder -> rsdd_bdd_ptr -> int64")]
pub fn bdd_model_count(builder: &'static RsddBddBuilder, bdd: &'static RsddBddPtr) -> u64 {
    let num_vars = builder.0.num_vars();
    let smoothed = builder.0.smooth(bdd.0, num_vars);
    let unweighted_params: WmcParams<FiniteField<{ primes::U64_LARGEST }>> =
        WmcParams::new(HashMap::from_iter(
            (0..num_vars as u64)
                .map(|v| (VarLabel::new(v), (FiniteField::one(), FiniteField::one()))),
        ));

    let mc = smoothed.unsmoothed_wmc(&unweighted_params).value();
    mc as u64
}
