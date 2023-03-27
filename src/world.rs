use crate::{ev::Control, ComponentManager, EntityManager, Ev, Scene, SystemManager};
use glium::{
    glutin::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
    Surface,
};

#[derive(Default)]
pub struct World<'a> {
    pub em: EntityManager,
    pub cm: ComponentManager<'a>,
}

impl World<'static> {
    pub fn update(
        &mut self,
        mut control: Control,
        flow: &mut ControlFlow,
        scene: &mut Scene,
        system_manager: &mut SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.update(&mut Ev::Event(&mut control), scene, self)?;

        if let Event::MainEventsCleared = &control.event {
            let mut target = scene.display.draw();

            let [r, g, b, a] = scene.bg;

            target.clear_color_and_depth((r, g, b, a), 1.0);

            system_manager.update(&mut Ev::Draw((&mut control, &mut target)), scene, self)?;

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
        mut scene: Scene,
        mut system_manager: SystemManager<'static>,
    ) -> anyhow::Result<()> {
        system_manager.init(&mut scene, &mut self)?;

        event_loop.run(move |event, _, control_flow| {
            if let Err(e) = self.update(
                Control::new(event),
                control_flow,
                &mut scene,
                &mut system_manager,
            ) {
                eprintln!("{}", e);
            }
        });
    }
}
