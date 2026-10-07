#[cfg(not(any(target_os = "linux", target_os = "macos")))]
compile_error!("Phoxal supports Linux and macOS only");

phoxal::api!();

mod config;
mod runtime;

fn main() -> phoxal::Result<()> {
    phoxal::runtime::run::<runtime::ZedF9p>()
}

#[cfg(test)]
mod model_tests;
