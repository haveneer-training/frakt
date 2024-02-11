use std::sync::mpsc::Receiver;

use blue_box::{utils::colors::color_palette, types::desc::PixelIntensity};
use pixels::{SurfaceTexture, Pixels, Error};
use winit::{
    dpi::{LogicalSize},
    window::WindowBuilder,
    event_loop::{EventLoop, ControlFlow},
    event::Event,
};
use winit_input_helper::WinitInputHelper;

pub struct DisplayFractal{
    width: u16,
    height: u16,
}

impl DisplayFractal{

    pub fn new(width: u16, height: u16) -> Self {
        DisplayFractal {
            width,
            height
        }
    }

    pub fn start(&self, rx: Receiver<Vec<PixelIntensity>>) -> Result<(), Error> {

        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        let window = {
            let size = LogicalSize::new(self.width as f64, self.height as f64);
            WindowBuilder::new()
                .with_title("Fractal")
                .with_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(self.width as u32, self.height as u32, surface_texture)?
        };

        event_loop.run(move |event, _, control_flow| {
            if let Ok(data_intensity) = rx.try_recv() {
                for intensity in data_intensity{
                    let color: [u8; 3] = [255, 0, 0];
                    let position = (100,100);
                    DisplayFractal::set_pixel_color(&mut pixels, position, color, 400);
                    DisplayFractal::set_pixel_color(&mut pixels, (200,200), color, 400);
                }
            }
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                if let Err(err) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        });
    }

    fn set_pixel_color(pixels: &mut Pixels, position: (usize, usize), color: [u8; 3], width: u32) {
        let (x, y) = position;
        let frame = pixels.frame_mut();
        let index = (y * width as usize + x) * 4;
        if index + 3 < frame.len() {
            frame[index] = 0xFF; // R
            frame[index + 1] = 0xFF; // G
            frame[index + 2] = 0xFF; // B
            frame[index + 3] = 0xFF; // A 
        }
    }

}
