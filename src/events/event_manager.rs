

// use crate::{BindEvent, Context, Display, Entity, Event, FontOrId, Propagation, State, Tree, TreeExt, Visibility, WindowEvent, entity};


use femtovg::{
    renderer::OpenGl, Canvas,
};

use crate::{Context, Event, Propagation, Tree, TreeExt};


/// Dispatches events to widgets.
/// 
/// The [EventManager] is responsible for taking the events in the event queue in state
/// and dispatching them to widgets based on the target and propagation metadata of the event.
/// The is struct is used internally by the application and should not be constructed directly.
pub struct EventManager {

    // Queue of events to be processed
    event_queue: Vec<Event>,

    // A copy of the tree for iteration
    tree: Tree,
}

impl EventManager {
    pub fn new() -> Self {
        EventManager {
            event_queue: Vec::new(),

            tree: Tree::new(),
        }
    }

    pub fn flush_events(&mut self, context: &mut Context) {

        // Clear the event queue in the event manager
        self.event_queue.clear();

        //state.removed_entities.clear();

        // Move events from state to event manager
        self.event_queue.extend(context.event_queue.drain(0..));

        // Sort the events by order
        //self.event_queue.sort_by_cached_key(|event| event.order);

        // Loop over the events in the event queue
        'events: for event in self.event_queue.iter_mut() {
            //println!("Event: {:?}", event);

            // Define the target to prevent multiple mutable borrows error
            let target = event.target;

            // Direct

            // Send event to target
            if let Some(mut container) = context.containers.remove(&event.target) {
                container.on_event_(context, event);

                context.containers.insert(event.target, container);

            } else if let Some(mut node) = context.nodes.remove(&event.target) {
                node.on_event_(context, event);

                context.nodes.insert(event.target, node);
            }
            
            if event.consumed {
                continue 'events;
            }

            // Propagate up from target to root (not including target)
            if event.propagation == Propagation::Up {
                // Walk up the tree from parent to parent
                for entity in target.parent_iter(&self.tree) {
                    // Skip the target entity
                    if entity == event.target {
                        continue;
                    }

                    // Send event to all entities before the target
                    if let Some(mut container) = context.containers.remove(&entity) {
                        container.on_event_(context, event);

                        context.containers.insert(entity, container);
                        
                        
                    } else if let Some(mut node) = context.nodes.remove(&entity) {
                        node.on_event_(context, event);

                        context.nodes.insert(entity, node);
                    }

                    // Skip to the next event if the current event is consumed
                    if event.consumed {
                        continue 'events;
                    }
                }
            }
        }
    }
}
