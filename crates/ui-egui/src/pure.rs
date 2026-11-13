//! The Pure edition (PureSheets): everything that is ours, in one place.
//!
//! This fork builds two apps from one tree. Without the cargo feature `pure` it is upstream's
//! GridCraft, unchanged. With it, the visible names, the palette, the sizes and a few pieces of
//! chrome come from this module. Upstream files reach it through short guards
//! (`if crate::pure::ON { … }`), so both sides always compile and a new upstream release merges
//! with conflicts of a line or two.

/// Whether this build is the Pure edition.
pub const ON: bool = cfg!(feature = "pure");

/// The app's visible name in the Pure edition.
pub const NAME: &str = "PureSheets (Craft)";

/// The upstream app this edition is built on, as About credits it.
pub const UPSTREAM: &str = "GridCraft";

/// The name the interface shows for the app: ours in the Pure edition, upstream's otherwise.
/// Message boxes and dialog titles pass the name upstream wrote through here.
pub fn app_name(upstream: &str) -> &str {
    if ON && upstream == UPSTREAM { NAME } else { upstream }
}
