trait MarkerAttribute {}
pub(crate) struct DeprecatedAttribute {}
impl MarkerAttribute for DeprecatedAttribute {}
pub(crate) struct SyntheticAttribute {}
impl MarkerAttribute for SyntheticAttribute {}