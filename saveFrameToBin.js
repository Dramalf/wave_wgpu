
export function saveFrameToBin(offset, size) {
  const bufferView = new Float32Array(window.WASM_MEMORY.buffer, offset, size);
  const blob = new Blob([bufferView]);
  saveAs(blob, "all_frames.bin");
}