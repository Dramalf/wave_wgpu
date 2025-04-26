

#   wave\_wgpu
<div align="center">
   <img src="demo.gif" width="300" height="300">
</div>

This project is a Rust-based implementation for simulating and visualizing wave phenomena using the `wgpu` graphics library. `wgpu` enables portable GPU computation and rendering, making it suitable for creating interactive and visually appealing wave simulations.

This project serves as an excellent self-study resource to learn about `wgpu`, WebAssembly (`wasm`), and compute shaders.

**Key Features:**

* **Wave Simulation:** Implements algorithms to model wave propagation, reflection, and interference.
* **GPU Acceleration:** Utilizes `wgpu` for efficient computation of wave dynamics on the GPU.
* **Visualization:** Renders the wave patterns.
* **Cross-Platform:** Supports both native Rust execution and web browser execution via WebAssembly.

**Potential Technologies Used:**

* **Rust:** The primary programming language.
* **wgpu:** For GPU computation and rendering.
* **wasm-bindgen:** To compile Rust to WebAssembly for web deployment.
* **ncview:** For viewing the NetCDF output files.
* **NetCDF format:** For data storage.
* **Python & Jupyter Notebook:** For post-processing of WebAssembly output.

**To Get Started:**

1.  **Prerequisites:**

    * Rust toolchain installed.
    * System dependencies for `wgpu` (these vary by platform).
    * `ncview` installed (for viewing output).
    * Python and Jupyter Notebook installed (if using the WebAssembly build).

2.  **Native Rust Execution:**

    * Clone the repository:

        ```bash
        git clone https://github.com/Dramalf/wave_wgpu
        cd wave_wgpu
        ```

    * Build and run the project:

        ```bash
        cargo run
        ```

    * This will generate `output_rs.nc`.
    * View the output using `ncview`:

        ```bash
        ncview output_rs.nc
        ```

3.  **Web Browser Execution (WebAssembly):**

    * Build the project for WebAssembly (you might need to configure your Rust toolchain for the `wasm32-unknown-unknown` target):

        ```bash
         wasm-pack build --target web 
        ```

    * Set up a local web server to serve the project files (e.g., using `python -m http.server` or `npx http-server`).
    * Access `http://localhost:8000/index.html` in your web browser.
    * This will generate `all_frames.bin`.
    * Use the provided `nc.ipynb` Jupyter Notebook to convert `allFrames.bin` to `output_wasm.nc`.
    * View the output using `ncview`:

        ```bash
        ncview output_wasm.nc
        ```
