use bytemuck::Zeroable;
use wgpu::util::DeviceExt;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const GRID_WIDTH: u32 = 40;
const GRID_HEIGHT: u32 = 40;
const NUM_FRAMES: u32 = 100;
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, Zeroable)]
struct WaveParams {
    frame: f32,
    tx: u32,
    ty: u32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Could't initialize logger");
        } else {
            env_logger::init();
        }
    }
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        #[cfg(not(target_arch = "wasm32"))]
        backends: wgpu::Backends::PRIMARY,
        #[cfg(target_arch = "wasm32")]
        backends: wgpu::Backends::GL,
        ..Default::default()
    });

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // WebGL doesn't support all of wgpu's features, so if
            // we're building for the web we'll have to disable some.
            required_limits: if cfg!(target_arch = "wasm32") {
                wgpu::Limits::downlevel_webgl2_defaults()
            } else {
                wgpu::Limits::default()
            },
            memory_hints: Default::default(),
            trace: wgpu::Trace::Off, // Trace path
        })
        .await
        .unwrap();
    let grid_size = (GRID_WIDTH * GRID_HEIGHT) as u64;
    let buffer_desc = |label| wgpu::BufferDescriptor {
        label: Some(label),
        size: grid_size * std::mem::size_of::<f32>() as u64,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    };
    let mut u_prev = device.create_buffer(&buffer_desc("u_prev"));
    let mut u_curr = device.create_buffer(&buffer_desc("u_curr"));
    let mut u_next = device.create_buffer(&buffer_desc("u_next"));

    let mut alpha_data = vec![0.0f32; (GRID_WIDTH * GRID_WIDTH) as usize];

    for y in 0..GRID_WIDTH {
        for x in 0..GRID_WIDTH {
            let idx = (y * GRID_WIDTH + x) as usize;
            alpha_data[idx] = 0.29*0.29 ;
        }
    }
    let alpha_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("alpha buffer"),
        contents: bytemuck::cast_slice(&alpha_data),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    let mut wave_params = WaveParams {
        frame:  0.0f32,
        tx: 5,
        ty: 5,
    };
    let payload_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("wave_params_buffer"),
        contents: bytemuck::bytes_of(&wave_params),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let shader = device.create_shader_module(wgpu::include_wgsl!("wave.wgsl"));
    let bind_group_layout_0 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("wave alpha bind group layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::COMPUTE,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: false },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });
    let bind_group_layout_1 = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("wave bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group_0 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("alpha bind group"),
        layout: &bind_group_layout_0,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: alpha_buffer.as_entire_binding(),
        }],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("pipeline_layout"),
        bind_group_layouts: &[&bind_group_layout_0, &bind_group_layout_1],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("wave_compute_pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader,
        entry_point: Some("main"),
        cache: None,
        compilation_options: Default::default(),
    });
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("staging_buffer"),
        size: grid_size * std::mem::size_of::<f32>() as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    for frame in 0..NUM_FRAMES {
        wave_params.frame = frame as f32;

        queue.write_buffer(&payload_buffer, 0, bytemuck::bytes_of(&wave_params));
        let bind_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("wave bind group"),
            layout: &bind_group_layout_1,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: u_prev.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: u_curr.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: u_next.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: payload_buffer.as_entire_binding(),
                },
            ],
        });
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("wave encoder"),
        });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("wave compute pass"),
                timestamp_writes: None,
            });
            cpass.set_pipeline(&compute_pipeline);
            cpass.set_bind_group(0, &bind_group_0, &[]);
            cpass.set_bind_group(1, &bind_group_1, &[]);
            cpass.dispatch_workgroups(GRID_WIDTH * GRID_HEIGHT, 1, 1);

        }
        encoder.copy_buffer_to_buffer(
            &u_curr,
            0,
            &staging_buffer,
            0,
            u_curr.size(),
        );
    
        queue.submit(Some(encoder.finish()));
        let buffer_slice = staging_buffer.slice(..);
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |r| sender.send(r).unwrap());
        device.poll(wgpu::PollType::wait()).unwrap();
        receiver.recv_async().await.unwrap().unwrap();
        println!("Result received.");
        {
            let view = buffer_slice.get_mapped_range();
            let data: &[f32] = bytemuck::cast_slice(&view);
            println!("Result: {:?}", &data);
        }
        staging_buffer.unmap();
        let tmp = u_prev;
        u_prev = u_curr;
        u_curr = u_next;
        u_next = tmp;
    }
}
