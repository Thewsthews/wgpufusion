use wgpu::{core::{device, instance}, util::DeviceExt, Adapter};
use pollster::block_on;
use image::{GenericImageView,DynamicImage};

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

}

fn main() {
    block_on(run_gpu_compute());
}