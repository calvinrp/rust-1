// issue: rust-lang/rust#160685
// Ambiguous globs of the same item that also cycle through `ambiguity_vis_max`
// (`axiomatic` and `own` glob-import each other; the item also arrives via `orphan`).
// Minimized from the `reflect_tools` crate.

#![feature(rustc_attrs)]
#![allow(internal_features)]
#![deny(dead_code)]

pub mod axiomatic {
    #[allow(unused_imports)]
    use super::*; // not pub
    pub use self::own::*;

    pub mod own {
        pub use super::*;
        pub use super::orphan::*;
    }

    pub mod orphan {
        pub use super::private::CollectionDescriptor;
    }

    // Private so the only public path is the glob reexport, matching
    // `ambiguous-import-visibility-globglob-reachable.rs`.
    mod private {
        #[rustc_effective_visibility]
        pub struct CollectionDescriptor {}
        //~^ ERROR Direct: pub(in crate::axiomatic), Reexported: pub, Reachable: pub, ReachableThroughImplTrait: pub
    }
}

pub use axiomatic::orphan::*;

fn main() {}
