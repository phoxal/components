# ddsm115 component

## Deterministic package qualification

Run `cargo test --locked`, `cargo fmt --check`, and `cargo clippy --locked --all-targets -- -D warnings` independently in this package.
Tests exercise the actual generated capability contracts and exact initialization refusal, and parse the authored model to verify native target identities, sensor references and packaged resources without MuJoCo.
The hardware backend is not implemented.
There is no device-write or measured-sample path to qualify, and these tests do not invent one or establish physical hardware support.
Native simulation substitution is a separate robot acceptance boundary.
