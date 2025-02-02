use wgpu::{core::{device::{self, WaitIdleError}, instance}, util::DeviceExt, Adapter};
use pollster::block_on;
use image::{buffer, DynamicImage, GenericImageView};

async fn run_gpu_compute(){
    //initializes the GPU
    let instance = wgpu::Instance::new(wgpu::BackendBit::all());
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default())
        .await.expect("Failed to find a GPU adapter");
     let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
        .await.expect("Failed to create GPU device");

    // This is the Image Processing handler (Glaussian Blur) ---

    let image = image::open("src/example.png").unwrap();
    let (width, height) = image.dimensions();
    let image_data = image.to_rgba8().into_raw();

    let input_image_buffer = device.craete_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Image Buffer"),
        contents: &image_data,
        usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_SRC,
    });

    let output_image_buffer = device.create_buffer(desciptor: &wgpu::BufferDescriptor {
        label: Some("Output Image Buffer"),
        size: (width * height * 4) as u64,
        usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    //This part is responsible for the loading of the shader

    let shader_code = include_str!("compute.wsgl");
    let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_code.into()),
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: None,
        module: &shader_module,
        entry_point: "gaussian_blur",
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(input_image_buffer.slice(..)),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Buffer(output_image_buffer.slice(..)),
            },
        ],
    });

    //This part is resposible for encoding of the GPU commands
    let mut encoder = device.create_command_encoder(desciptor: &wgpu::CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });

        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch(width / 8, height / 8, 1);
    }

    //This copies the result to the CPU-readable buffer
    let staging_buffer = device.create_buffer(desciptor: &wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: (width * height * 4) as u64,
        usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(&output_image_buffer, 0, &staging_buffer, 0, output_image_buffer.size());
    queue.submit(std::iter::once(encoder.finish()));

    //This part is responsible for reading the results

    let buffer_slice = staging_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read);

    device.poll(wgpu::Maintain::Wait);
    let data = buffer_slice.get_mapped_range().to_vec();
    let output_img = image::RgbaImage::from_raw(width, height, data).expect("Failed to create image");
    
}

fn main() {
    block_on(run_gpu_compute());
}