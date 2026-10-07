use anyhow::Result;
use anyhow::anyhow;
use phoxal::runtime::Context;

const BACKEND_UNAVAILABLE: &str = "ddsm115 hardware backend unavailable: refusing to model motor or publish fabricated encoder measurements";

/// The DDSM115 hardware component driver. The endpoint surface is the
/// derived standard surface of the component's declared capabilities: a
/// velocity-commanded motor implies the leased actuator setpoint input and
/// the encoder implies the queued encoder sample output.
pub struct Ddsm115;

/// The DDSM115's endpoint contract: the derived standard surface.
#[phoxal::endpoints]
pub struct Ddsm115Api {}

#[phoxal::runtime(contract = Ddsm115Api, period_ms = 20, timeout_ms = 100, init_timeout_ms = 1_000)]
impl Ddsm115 {
    #[init]
    fn new(_config: ()) -> Result<Self> {
        Err(anyhow!(BACKEND_UNAVAILABLE))
    }

    #[step]
    fn unavailable(&mut self, _ctx: &mut Context<'_, Self>) -> Result<()> {
        Err(anyhow!(BACKEND_UNAVAILABLE))
    }
}

#[cfg(test)]
mod tests {
    use super::BACKEND_UNAVAILABLE;
    use phoxal::contracts::MethodShape;
    use phoxal::runtime::Harness;
    use phoxal::runtime::RuntimeContract;
    use phoxal::runtime::input::InputSet;
    use phoxal::runtime::outputs::OutputSet;

    #[test]
    fn capabilities_determine_the_standard_endpoints() {
        let encoder = crate::api::service_methods::u0::ENCODER.signature();
        assert_eq!(encoder.endpoint, "encoder");
        assert_eq!(encoder.service, "phoxal.robotics.v1.EncoderSample");
        assert_eq!(encoder.shape, MethodShape::Observation);
        let actuator = crate::api::service_methods::u0::ACTUATOR.signature();
        assert_eq!(actuator.endpoint, "actuator");
        assert_eq!(
            actuator.request,
            "phoxal.component.actuator.v1.ActuatorCommand"
        );
        assert_eq!(actuator.lease.map(|lease| lease.valid_for_ms()), Some(100));
        let inputs = &<super::Ddsm115Api as RuntimeContract>::Inputs::FIELDS;
        assert_eq!(inputs[0].name, "actuator");
        assert_eq!(
            <super::Ddsm115Api as RuntimeContract>::Outputs::FIELDS[0].port,
            Some("encoder")
        );
        // Standard types carry no compiled descriptor set; the vocabulary's
        // schema frames are retained instead, exactly like a Rust contract.
        assert!(crate::api::service_methods::u0::retain_standard_schemas() > 0);
        let error = match Harness::<super::Ddsm115>::new(serde_json::Value::Null) {
            Ok(_) => panic!("the hardware backend is unavailable"),
            Err(error) => error,
        };
        assert!(error.to_string().contains(BACKEND_UNAVAILABLE));
    }
}
