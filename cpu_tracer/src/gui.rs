use std::time::Instant;
use imgui::{FontSource, MouseCursor};
use imgui_wgpu::{Renderer, RendererConfig};
use imgui_winit_support::WinitPlatform;
use wgpu::{Queue, SurfaceConfiguration};
use winit::window::Window;

pub struct GUI {
    pub platform: WinitPlatform,
    pub imgui: imgui::Context,
    pub imgui_renderer: Renderer,
    last_cursor: Option<MouseCursor>,
    last_frame: Instant,
}

impl GUI {
    pub fn new(window: &Window, surface_cap: &SurfaceConfiguration,
               device: &wgpu::Device, queue: &Queue)
               -> Option<Self> {

        let mut imgui = imgui::Context::create();
        let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
        platform.attach_window(
            imgui.io_mut(),
            &window,
            imgui_winit_support::HiDpiMode::Default,
        );
        imgui.set_ini_filename(std::path::PathBuf::from("gpu_tracer/imgui.ini"));

        let hidpi_factor = window.scale_factor();
        let font_size = (13.0 * hidpi_factor) as f32;
        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        imgui.fonts().add_font(&[FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            }),
        }]);

        let renderer_config = RendererConfig {
            texture_format: surface_cap.format,
            ..Default::default()
        };

        let mut imgui_renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);

        let mut last_frame = Instant::now();

        Some(Self {
            platform,
            imgui,
            imgui_renderer,
            last_cursor: None,
            last_frame,
        })
    }

    pub fn display_ui(&mut self, window: &Window, progress: f32, compute_time: f64) {
        let dt = self.last_frame.elapsed().as_secs_f32();
        let now = Instant::now();

        // fps_counter.update(dt);
        // fly_camera_controller.after_events(render_params.viewport_size, 2.0 * dt);

        self.imgui.io_mut().update_delta_time(now - self.last_frame);

        self.last_frame = now;
        let average_fps = 4f32;
        let mut spp = 1000u32;
        let mut fov = 10f64;
        {
            self.platform
                .prepare_frame(self.imgui.io_mut(), &window)
                .expect("WinitPlatform::prepare_frame failed");

            let ui = self.imgui.frame();
            {
                let window = ui.window("Parameters");
                window
                    .size([300.0, 300.0], imgui::Condition::FirstUseEver)
                    .build(|| {
                        ui.text(format!(
                            "Avg compute time: {:.3}ms, render progress: {:.1} %",
                            compute_time,
                            // fps_counter.average_fps(),
                            progress * 100.0
                        ));
                        ui.separator();

                        ui.text("Sampling parameters");

                        ui.text("samples per pixel per frame");
                        ui.same_line();
                        ui.radio_button(
                            "1",
                            &mut spp,
                            // &mut render_params.sampling.num_samples_per_pixel,
                            1_u32,
                        );
                        ui.same_line();
                        ui.radio_button(
                            "4",
                            &mut spp,
                            4_u32,
                        );
                        ui.same_line();
                        ui.radio_button(
                            "8",
                            &mut spp,
                            8_u32,
                        );

                        ui.text("total samples per pixel");
                        ui.same_line();
                        ui.radio_button(
                            "128",
                            &mut spp,
                            128_u32,
                        );
                        ui.same_line();
                        ui.radio_button(
                            "256",
                            &mut spp,
                            256_u32,
                        );
                        ui.same_line();
                        ui.radio_button(
                            "512",
                            &mut spp,
                            512_u32,
                        );

                        ui.slider(
                            "num bounces",
                            4,
                            10,
                            &mut spp,
                        );

                        ui.separator();

                        ui.text("Camera parameters");
                        ui.slider(
                            "vfov",
                            10.0,
                            90.0,
                            &mut fov,
                        );
                        ui.slider(
                            "aperture",
                            0.0,
                            1.0,
                            &mut fov,
                        );
                        ui.slider(
                            "focus distance",
                            5.0,
                            20.0,
                            &mut fov,
                        );

                        ui.separator();

                        ui.text("Sky parameters");
                        // ui.slider(
                        //     "azimuth",
                        //     0_f32,
                        //     360_f32,
                        //     & mut fov,
                        // );
                        // ui.slider(
                        //     "inclination",
                        //     0_f32,
                        //     90_f32,
                        //     & mut render_params.sky.zenith_degrees,
                        // );
                        // ui.slider(
                        //     "turbidity",
                        //     1_f32,
                        //     10_f32,
                        //     & mut render_params.sky.turbidity,
                        // );
                        // ui.color_edit3("albedo", & mut render_params.sky.albedo);
                    });
            }

            if self.last_cursor != ui.mouse_cursor() {
                self.last_cursor = ui.mouse_cursor();
                self.platform.prepare_render(&ui, &window);
            }
        }
    }
}
