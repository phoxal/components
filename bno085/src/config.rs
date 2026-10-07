/// The component driver's authored configuration.
///
/// The hardware transport is not available in this framework release, so the
/// driver deliberately accepts an empty typed configuration.
#[derive(Debug, Default, serde::Deserialize, phoxal::Config)]
#[serde(deny_unknown_fields)]
pub struct Bno085Config {}
