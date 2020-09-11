pub use crate::logger;

use winit::{
      event::{Event, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::{Window,WindowBuilder, WindowId},
};

pub trait AppEventHandlerFactory {
    fn create_event_handler(&self, window: Window) -> Box<dyn AppEventHandler>;
}

pub trait AppEventHandler {
    /// (will only be run for continuos apps)
    fn on_update(&mut self) {}
    fn on_window_resize(&mut self, width: u32, height: u32);
    fn on_redraw(&mut self);
    fn on_window_exit(&mut self);
}

pub enum UpdateFrequency {
    /// only update when event needs to be handle (appropriate for GUI/static apps)
    OnEvent,
    /// continualy run update loop (appropriate for game-type apps)
    // @TODO - should on_update call back be part of continuous???
    Continuous,
}

pub struct App {
    update_frequency: UpdateFrequency,
    window_id: WindowId,
    event_loop: EventLoop<()>,
    event_handler: Box<dyn AppEventHandler>,
}

impl App {

    pub fn new(
        update_frequency: UpdateFrequency,
        event_handler_factory: Box<dyn AppEventHandlerFactory>,
    ) -> App {
        logger::init();

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let window_id = window.id();

        let event_handler = event_handler_factory.create_event_handler(window);

        App {
            update_frequency,
            window_id,
            event_loop,
            event_handler,
        }
    }

    /// Blocks until Application is complete
    pub fn run(self) {

        let update_type = self.update_frequency;
        let mut event_handler = self.event_handler;
        let my_window_id = self.window_id;

        self.event_loop.run(move |event, _, control_flow| {

            *control_flow = match update_type {
                UpdateFrequency::Continuous => {
                    event_handler.on_update();
                    ControlFlow::Poll
                },
                UpdateFrequency::OnEvent => ControlFlow::Wait,
            };

            match event {
                Event::WindowEvent {
                    event,
                    window_id,
                } if window_id == my_window_id => {
                    match event {
                        WindowEvent::CloseRequested => {
                            event_handler.on_window_exit();
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(size) => event_handler.on_window_resize(size.width, size.height),
                        _ => ()
                    }
                }
                Event::RedrawEventsCleared => event_handler.on_redraw(),
                _ => (),
            }
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_app() {
        //let app = App::new(
    }
}
