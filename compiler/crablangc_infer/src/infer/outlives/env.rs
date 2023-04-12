use crate::infer::free_regions::FreeRegionMap;
use crate::infer::GenericKind;
use crate::traits::query::OutlivesBound;
use crablangc_data_structures::fx::FxIndexSet;
use crablangc_data_structures::transitive_relation::TransitiveRelationBuilder;
use crablangc_middle::ty::{self, Region};

use super::explicit_outlives_bounds;

/// The `OutlivesEnvironment` collects information about what outlives
/// what in a given type-checking setting. For example, if we have a
/// where-clause like `where T: 'a` in scope, then the
/// `OutlivesEnvironment` would record that (in its
/// `region_bound_pairs` field). Similarly, it contains methods for
/// processing and adding implied bounds into the outlives
/// environment.
///
/// Other code at present does not typically take a
/// `&OutlivesEnvironment`, but rather takes some of its fields (e.g.,
/// `process_registered_region_obligations` wants the
/// region-bound-pairs). There is no mistaking it: the current setup
/// of tracking region information is quite scattered! The
/// `OutlivesEnvironment`, for example, needs to sometimes be combined
/// with the `middle::RegionRelations`, to yield a full picture of how
/// (lexical) lifetimes interact. However, I'm reluctant to do more
/// refactoring here, since the setup with NLL is quite different.
/// For example, NLL has no need of `RegionRelations`, and is solely
/// interested in the `OutlivesEnvironment`. -nmatsakis
#[derive(Clone)]
pub struct OutlivesEnvironment<'tcx> {
    pub param_env: ty::ParamEnv<'tcx>,
    free_region_map: FreeRegionMap<'tcx>,

    // Contains the implied region bounds in scope for our current body.
    //
    // Example:
    //
    // ```
    // fn foo<'a, 'b, T>(x: &'a T, y: &'b ()) {
    //   bar(x, y, |y: &'b T| { .. } // body B1)
    // } // body B0
    // ```
    //
    // Here, when checking the body B0, the list would be `[T: 'a]`, because we
    // infer that `T` must outlive `'a` from the implied bounds on the
    // fn declaration.
    //
    // For the body B1 however, the list would be `[T: 'a, T: 'b]`, because we
    // also can see that -- within the closure body! -- `T` must
    // outlive `'b`. This is not necessarily true outside the closure
    // body, since the closure may never be called.
    region_bound_pairs: RegionBoundPairs<'tcx>,
}

/// Builder of OutlivesEnvironment.
#[derive(Debug)]
struct OutlivesEnvironmentBuilder<'tcx> {
    param_env: ty::ParamEnv<'tcx>,
    region_relation: TransitiveRelationBuilder<Region<'tcx>>,
    region_bound_pairs: RegionBoundPairs<'tcx>,
}

/// "Region-bound pairs" tracks outlives relations that are known to
/// be true, either because of explicit where-clauses like `T: 'a` or
/// because of implied bounds.
pub type RegionBoundPairs<'tcx> =
    FxIndexSet<ty::OutlivesPredicate<GenericKind<'tcx>, Region<'tcx>>>;

impl<'tcx> OutlivesEnvironment<'tcx> {
    /// Create a builder using `ParamEnv` and add explicit outlives bounds into it.
    fn builder(param_env: ty::ParamEnv<'tcx>) -> OutlivesEnvironmentBuilder<'tcx> {
        let mut builder = OutlivesEnvironmentBuilder {
            param_env,
            region_relation: Default::default(),
            region_bound_pairs: Default::default(),
        };

        builder.add_outlives_bounds(explicit_outlives_bounds(param_env));

        builder
    }

    #[inline]
    /// Create a new `OutlivesEnvironment` without extra outlives bounds.
    pub fn new(param_env: ty::ParamEnv<'tcx>) -> Self {
        Self::builder(param_env).build()
    }

    /// Create a new `OutlivesEnvironment` with extra outlives bounds.
    pub fn with_bounds(
        param_env: ty::ParamEnv<'tcx>,
        extra_bounds: impl IntoIterator<Item = OutlivesBound<'tcx>>,
    ) -> Self {
        let mut builder = Self::builder(param_env);
        builder.add_outlives_bounds(extra_bounds);
        builder.build()
    }

    /// Borrows current value of the `free_region_map`.
    pub fn free_region_map(&self) -> &FreeRegionMap<'tcx> {
        &self.free_region_map
    }

    /// Borrows current `region_bound_pairs`.
    pub fn region_bound_pairs(&self) -> &RegionBoundPairs<'tcx> {
        &self.region_bound_pairs
    }
}

impl<'tcx> OutlivesEnvironmentBuilder<'tcx> {
    #[inline]
    #[instrument(level = "debug")]
    fn build(self) -> OutlivesEnvironment<'tcx> {
        OutlivesEnvironment {
            param_env: self.param_env,
            free_region_map: FreeRegionMap { relation: self.region_relation.freeze() },
            region_bound_pairs: self.region_bound_pairs,
        }
    }

    /// Processes outlives bounds that are known to hold, whether from implied or other sources.
    fn add_outlives_bounds<I>(&mut self, outlives_bounds: I)
    where
        I: IntoIterator<Item = OutlivesBound<'tcx>>,
    {
        // Record relationships such as `T:'x` that don't go into the
        // free-region-map but which we use here.
        for outlives_bound in outlives_bounds {
            debug!("add_outlives_bounds: outlives_bound={:?}", outlives_bound);
            match outlives_bound {
                OutlivesBound::RegionSubParam(r_a, param_b) => {
                    self.region_bound_pairs
                        .insert(ty::OutlivesPredicate(GenericKind::Param(param_b), r_a));
                }
                OutlivesBound::RegionSubAlias(r_a, alias_b) => {
                    self.region_bound_pairs
                        .insert(ty::OutlivesPredicate(GenericKind::Alias(alias_b), r_a));
                }
                OutlivesBound::RegionSubRegion(r_a, r_b) => match (*r_a, *r_b) {
                    (
                        ty::ReStatic | ty::ReEarlyBound(_) | ty::ReFree(_),
                        ty::ReStatic | ty::ReEarlyBound(_) | ty::ReFree(_),
                    ) => self.region_relation.add(r_a, r_b),
                    (ty::ReError(_), _) | (_, ty::ReError(_)) => {}
                    // FIXME(#109628): We shouldn't have existential variables in implied bounds.
                    // Panic here once the linked issue is resolved!
                    (ty::ReVar(_), _) | (_, ty::ReVar(_)) => {}
                    _ => bug!("add_outlives_bounds: unexpected regions: ({r_a:?}, {r_b:?})"),
                },
            }
        }
    }
}
