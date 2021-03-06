use polyhorn_core::{CommandBuffer as _, Compositor as _};
use polyhorn_ui::geometry::Size;

use crate::raw::{Animator, CommandBuffer, Compositor, ContainerID};
use crate::Reference;

/// Platform-specific implementation of the view handle trait that can be used
/// to execute imperative code against a view.
pub struct ViewHandle {
    pub(crate) container_id: Reference<ContainerID>,
    pub(crate) compositor: Compositor,
}

impl polyhorn_ui::handles::ViewHandle for ViewHandle {
    fn layout_guide(&self) -> polyhorn_ui::layout::LayoutGuide<f32> {
        unimplemented!("Help")
    }

    fn size_with_buffer<F>(&mut self, buffer: &mut CommandBuffer, callback: F)
    where
        F: FnOnce(Size<f32>) + Send + 'static,
    {
        let container_id = match self.container_id.to_owned() {
            Some(container_id) => container_id,
            None => panic!("Can't measure view that has not yet been mounted."),
        };

        buffer.mutate(&[container_id], |containers| {
            if let Some(view) = containers[0].container().to_view() {
                let frame = view.frame();
                callback(Size {
                    width: frame.size.width as _,
                    height: frame.size.height as _,
                });
            }
        });
    }
}

impl polyhorn_ui::animation::Animatable for ViewHandle {
    type Animator = Animator;
    type CommandBuffer = CommandBuffer;

    fn animate<F>(&mut self, animations: F)
    where
        F: FnOnce(&mut Animator) + Send + 'static,
    {
        // Create a new command buffer.
        let mut buffer = self.compositor.buffer();

        self.animate_with_buffer(&mut buffer, animations);

        // And finally, commit the command buffer to synchronize the mutation.
        buffer.commit();
    }

    fn animate_with_buffer<F>(&mut self, buffer: &mut CommandBuffer, animations: F)
    where
        F: FnOnce(&mut Animator) + Send + 'static,
    {
        let container_id = match self.container_id.to_owned() {
            Some(container_id) => container_id,
            None => panic!("Can't animate view that has not yet been mounted."),
        };

        // Add a mutation of the container to the command buffer.
        buffer.mutate(&[container_id], |containers| {
            if let Some(view) = containers[0].container().to_view() {
                animations(&mut Animator::new(view));
            }
        });
    }
}
