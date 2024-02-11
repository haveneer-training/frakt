use pixels::{SurfaceTexture, Pixels, Error};
use winit::{
    dpi::LogicalSize,
    window::WindowBuilder,
    event_loop::{EventLoop, ControlFlow},
    event::{
        Event, 
        VirtualKeyCode
    }
};
use winit_input_helper::WinitInputHelper;

pub struct DisplayFractal{
    width: u32,
    height: u32,
}

impl DisplayFractal{

    pub fn new() -> Self {
        DisplayFractal {
            width: 400,
            height: 400
        }
    }

    pub fn start(&self) -> Result<(), Error> {

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
            Pixels::new(self.width, self.height, surface_texture)?
        };

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                if let Err(err) = pixels.render() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
        });

    }
}
