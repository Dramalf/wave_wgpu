struct Params {
frame: f32,
    tx: u32,
    ty: u32,
};

const GRID_WIDTH: u32 = 40u;
@group(0) @binding(0)
var<storage, read_write> alpha: array<f32>;
@group(1) @binding(0)
var<storage, read_write> u_prev: array<f32>;
@group(1) @binding(1)
var<storage, read_write> u_curr: array<f32>;
@group(1) @binding(2)
var<storage, read_write> u_next: array<f32>;
@group(1) @binding(3)
var<uniform> params: Params;

const PI: f32 = 3.141592653589793;

fn curr_v(x: u32, y: u32) -> f32 {
    let index = x * GRID_WIDTH + y;
    if x < GRID_WIDTH && x >= 0u && y < GRID_WIDTH && y >= 0u {
        return u_curr[index];
    }
    return 0f;
}
fn prev_v(x: u32, y: u32) -> f32 {
    let index = x * GRID_WIDTH + y;
    if x < GRID_WIDTH && x >= 0u && y < GRID_WIDTH && y >= 0u {
        return u_prev[index];
    }
    return 0f;
}
fn next_v(x: u32, y: u32) -> f32 {
    let index = x * GRID_WIDTH + y;
    if x < GRID_WIDTH && x >= 0u && y < GRID_WIDTH && y >= 0u {
        return u_next[index];
    }
    return 0f;
}
fn alp_v(x: u32, y: u32) -> f32 {
    let index = x * GRID_WIDTH + y;
    if x < GRID_WIDTH && x >= 0u && y < GRID_WIDTH && y >= 0u {
        return alpha[index];
    }
    return 0f;
}
@compute @workgroup_size(1)
fn main(
    @builtin(global_invocation_id)
  global_id: vec3u,
) {
    let x = global_id.x / GRID_WIDTH;
    let y = global_id.x % GRID_WIDTH;
    let frame =params.frame;
    let tx = params.tx;
    let ty = params.ty;
    if global_id.x == tx * GRID_WIDTH + ty {
        let v = 10f * sin(2f * PI * frame / 8f);

        u_curr[global_id.x] = v;
        u_prev[global_id.x] = v;
    }

    u_next[global_id.x] = 
    alp_v(x,y) * (
        curr_v(x-1u,y) +
        curr_v(x+1u,y) +
        curr_v(x,y-1u) +
        curr_v(x,y+1u) -
        curr_v(x,y) * 4.0
    ) + 2.0 * curr_v(x,y) - prev_v(x,y);
}