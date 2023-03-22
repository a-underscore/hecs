use crate::{ev::Control, ComponentManager, EntityManager, Ev, SystemManager};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
    Display, Surface,
};

pub struct World<'a> {
    pub em: EntityManager,
    pub cm: ComponentManager<'a>,
    pub display: Display,
    pub bg: [f32; 4],
}

impl<'a> World<'a> {
    pub fn new(display: Display, bg: [f32; 4]) -> Self {
        Self {
            em: EntityManager::default(),
            cm: ComponentManager::default(),
            display,
            bg,
        }
    }
}

impl World<'static> {
    pub fn update(
        &mut self,
        mut control: Control,
        flow: &mut ControlFlow,
        system_manager: &mut SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.update(&mut Ev::Event(&mut control), self)?;

        if let Event::MainEventsCleared = &control.event {
            let mut target = self.display.draw();

            let [r, g, b, a] = self.bg;

            target.clear_color_and_depth((r, g, b, a), 1.0);

            system_manager.update(&mut Ev::Draw((&mut control, &mut target)), self)?;

            target.finish()?;
        }

        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = &control.event
        {
            *flow = ControlFlow::Exit;
        } else if let Some(control_flow) = control.flow {
            *flow = control_flow;
        } else {
            *flow = ControlFlow::Poll;
        }

        Ok(())
    }

    pub fn init(
        mut self,
        event_loop: EventLoop<()>,
        mut system_manager: SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.init(&mut self)?;

        event_loop.run(move |event, _, control_flow| {
            if let Err(e) = self.update(Control::new(event), control_flow, &mut system_manager) {
                eprintln!("{}", e);
            }
        });
    }
}
