use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WebCanvasWindowHandle, WebDisplayHandle};
use std::ptr::NonNull;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;

pub async fn run() -> Result<(), wasm_bindgen::JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let width = canvas.width();
    let height = canvas.height();

    let instance = wgpu::Instance::default();

    let raw_display_handle = RawDisplayHandle::Web(WebDisplayHandle::new());
    let canvas_handle = WebCanvasWindowHandle::new(
        NonNull::from(&canvas as &wasm_bindgen::JsValue).cast(),
    );
    let raw_window_handle = RawWindowHandle::WebCanvas(canvas_handle);
    let surface_target = wgpu::SurfaceTargetUnsafe::RawHandle {
        raw_display_handle,
        raw_window_handle,
    };
    let surface = unsafe {
        instance
            .create_surface_unsafe(surface_target)
            .map_err(|e| wasm_bindgen::JsValue::from(e.to_string()))?
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor::default()
        )
        .await
        .unwrap();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_capabilities(&adapter).formats[0],
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        desired_maximum_frame_latency: 2,
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    // Simple clear pass
    let frame = surface.get_current_texture().unwrap();
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

    {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    queue.submit(Some(encoder.finish()));
    frame.present();

    Ok(())
}