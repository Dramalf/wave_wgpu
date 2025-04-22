use tutorial1_window::run;

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(run());
    #[cfg(target_arch = "wasm32")]
    run();
}