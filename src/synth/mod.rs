pub mod errors;
pub mod params;

/// Empty input or an unusable parameter block.
pub const ERR_ARG: i32 = -1;
/// Unknown `AqVersion`.
pub const ERR_VERSION: i32 = -2;
/// The engine rejected the call or no voice matched.
pub const ERR_OPEN: i32 = -3;

