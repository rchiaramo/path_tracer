use crate::bvh_node::BVHTree;
use crate::camera::Camera;
use crate::compute_shader::ComputeShader;
use crate::gpu_buffer::GPUBuffer;
use crate::gpu_structs::{GPUCamera, GPUSamplingParameters};
use crate::gui::GUI;
use crate::parameters::{RenderParameters, RenderProgress, SamplingParameters};
use crate::scene::Scene;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupLayoutDescriptor, BufferAddress, BufferUsages, Device, Queue, RenderPipeline, ShaderStages, Surface, TextureFormat};
use winit::event::WindowEvent;

pub struct PathTracer {
    image_buffer: GPUBuffer,
    frame_buffer: GPUBuffer,
    camera_buffer: GPUCamera,
    sampling_parameters_buffer: GPUSamplingParameters,
    projection_buffer: [[f32;4];4],
    view_buffer: [[f32;4];4],
    display_bind_group: BindGroup,
    display_pipeline: RenderPipeline,
    render_parameters: RenderParameters,
    last_render_parameters: RenderParameters,
    render_progress: RenderProgress,
    compute_shader: ComputeShader
}

impl PathTracer {
    pub fn new(device: &Device,
               max_window_size: u32,
               window_size: (u32, u32)) 
        -> Option<Self> {
        // create the image_buffer that the compute shader will use to store image
        // we make this array as big as the largest possible window on resize
        let image = vec![[0.0f32; 3]; max_window_size as usize];
        let image_buffer =
            GPUBuffer::new_from_bytes(device,
                                      BufferUsages::STORAGE,
                                      0u32,
                                      bytemuck::cast_slice(image.as_slice()),
                                      Some("image buffer"));

        // create the frame_buffer
        let frame_buffer = GPUBuffer::new(device,
                                          BufferUsages::UNIFORM,
                                          16 as BufferAddress,
                                          1u32,
                                          Some("frame buffer"));

        // group image and frame buffers into image bind group
        // for the display shader
        let display_bind_group_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("display bind group layout"),
                entries: &[
                    image_buffer.layout(ShaderStages::FRAGMENT, true),
                    frame_buffer.layout(ShaderStages::FRAGMENT, true)
                ],
            }
        );

        let display_bind_group = device.create_bind_group(
            &BindGroupDescriptor {
                label: Some("display bind group"),
                layout: &display_bind_group_layout,
                entries: &[
                    image_buffer.binding(),
                    frame_buffer.binding()
                ],
            }
        );

        // create the scene and the bvh_tree that corresponds to it
        let mut scene = Scene::new();
        let mut bvh_tree= BVHTree::new(scene.spheres.len());
        bvh_tree.build_bvh_tree(&mut scene.spheres);
        
        let spheres_buffer = scene.spheres;
        let materials_buffer = scene.materials;
        let bvh_buffer = bvh_tree.tree;


        // create the parameters bind group to interact with GPU during runtime
        // this will include the camera, and the sampling parameters
        // let lookAt = Vec3::new(0.0, 0.0, -1.0);
        // let lookFrom = Vec3::new(-2.0, 2.0, 1.0);
        // let camera = Camera::new(lookAt, lookFrom, 90.0, 0.0,3.4);
        let camera = Camera::default();
        let camera_buffer = GPUCamera::new(&camera, window_size);
        

        let sampling_parameters = SamplingParameters::default();
        let sampling_parameters_buffer =
            GPUSamplingParameters::get_gpu_sampling_params(&sampling_parameters);
        
        let ar = window_size.0 as f32 / window_size.1 as f32;
        let projection_buffer = camera.projection_transform(ar, 0.1, 100.0);
        let view_buffer = camera.view_transform();
        

        let render_parameters =
            RenderParameters::new(camera, sampling_parameters, window_size);
        let last_render_parameters = render_parameters.clone();
        let render_progress = RenderProgress::default();
        
        let compute_shader = ComputeShader::new(spheres_buffer, 
                                                materials_buffer, 
                                                bvh_buffer, 
                                                camera_buffer, 
                                                projection_buffer, 
                                                view_buffer,
                                                *render_parameters.sampling_parameters(),
                                                max_window_size);

        let shader = device.create_shader_module(
            wgpu::include_wgsl!("screen_shader.wgsl")
        );

        let display_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("display pipeline layout"),
                bind_group_layouts: &[&display_bind_group_layout],
                push_constant_ranges: &[],
            });

        let display_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("display pipeline"),
            layout: Some(&display_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: TextureFormat::Bgra8Unorm,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState{
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        Some(Self {
            image_buffer,
            frame_buffer,
            camera_buffer,
            sampling_parameters_buffer,
            projection_buffer,
            view_buffer,
            display_bind_group,
            display_pipeline,
            render_parameters,
            last_render_parameters,
            render_progress,
            compute_shader
        })

    }

    pub fn progress(&self) -> f32 {
        self.render_progress.progress()
    }

    pub fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    pub fn get_render_parameters(&self) -> RenderParameters {
        self.render_parameters.clone()
    }

    pub fn set_render_parameters(&mut self, render_parameters: RenderParameters) {
        self.render_parameters = render_parameters
    }

    pub fn update_buffers(&mut self, _queue: &Queue) {
        // if rp is the same as the stored buffer, no need to do anything
        if self.render_parameters == self.last_render_parameters {
            return;
        }

        // eventually we may have other things (e.g. sky) to update here
        let gpu_camera
            = GPUCamera::new(&self.render_parameters.camera(), self.render_parameters.get_viewport());
        self.camera_buffer = gpu_camera;
        
        let (h,w) = self.render_parameters.get_viewport();
        let ar = w as f32 / h as f32;
        let proj_mat = self.render_parameters.camera().projection_transform(ar, 0.1, 100.0);
        let view_mat = self.render_parameters.camera().view_transform();

        self.projection_buffer = proj_mat;
        self.view_buffer = view_mat;

        self.render_progress.reset();
    }

    pub fn run_compute_kernel(&mut self, _device: &Device, queue: &Queue) { //, queries: &mut Queries) {
        let size = self.render_parameters.get_viewport();

        let frame = self.render_progress.get_next_frame(&mut self.render_parameters);
        self.last_render_parameters = self.get_render_parameters();
        self.frame_buffer.queue_for_gpu(queue, bytemuck::cast_slice(&[frame]));

        let gpu_sampling_parameters
            = GPUSamplingParameters::get_gpu_sampling_params(self.render_parameters.sampling_parameters());
        self.sampling_parameters_buffer = gpu_sampling_parameters;

        self.compute_shader.run_render(queue, size, frame.into_array(), &mut self.image_buffer);
    }

    pub fn run_display_kernel(&mut self, surface: &mut Surface,
                              device: &Device, queue: &Queue, gui: &mut GUI) {

        let output = surface.get_current_texture().unwrap();
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("display kernel encoder"),
            });

        {
            let mut display_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("display render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None
                });
            display_pass.set_pipeline(&self.display_pipeline);
            display_pass.set_bind_group(0, &self.display_bind_group, &[]);
            display_pass.draw(0..6, 0..1);

            gui.imgui_renderer.render(
                gui.imgui.render(), queue, device, &mut display_pass
            ).expect("failed to render gui");
        }
        queue.submit(Some(encoder.finish()));
        output.present();
    }
}