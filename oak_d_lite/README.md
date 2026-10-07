# OAK-D Lite component

The component mount uses +X forward, +Y left, and +Z up.
Its native cameras look forward along mount +X, with image right along -Y and image up along +Z.
The authored left and right camera centers are separated by 75 mm; the left camera is on +Y.
These simplified model parameters do not constitute device calibration or a stereo reconstruction algorithm.

MuJoCo cameras look along camera -Z, so their MJCF `xyaxes` explicitly maps the camera frame into this mount convention.
The native component acceptance renders the actual model toward a target at known distance and checks forward depth, image up, and the left/right baseline.
The driver package declares its component-specific Rust contract beside the runtime, while `component.yaml` provides standard camera capability endpoints.
Its native model remains a component asset, and the selected driver process retains hardware behavior without a communication-only library.

## Deterministic package qualification

Run `cargo test --locked`, `cargo fmt --check`, and `cargo clippy --locked --all-targets -- -D warnings` independently in this package.
Tests exercise the actual generated capability contracts and exact initialization refusal, and parse the authored model to verify native target identities, sensor references and packaged resources without MuJoCo.
The hardware backend is not implemented.
There is no device-write or measured-sample path to qualify, and these tests do not invent one or establish physical hardware support.
Native simulation substitution is a separate robot acceptance boundary.
