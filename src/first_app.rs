use winit::{event_loop::EventLoop, window::WindowBuilder};

#[derive(Debug)]
pub struct FirstApp {
    pub w: u32,
    pub h: u32,
    pub window_name: &'static str,
}

pub trait Run {
    fn run(&self);
}

impl Run for FirstApp {
    fn run(&self) {
        let event_loop = EventLoop::new();
        let _window = WindowBuilder::new()
            .with_title(self.window_name)
            .with_inner_size(winit::dpi::LogicalSize::new(self.w, self.h))
            .with_resizable(true)
            .build(&event_loop)
            .unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Wait;

            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => (),
            }
        });
    }
}
