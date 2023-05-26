use std::collections::HashMap;

pub type EventCallback = Box<dyn FnMut(&[Box<dyn std::any::Any>])>;

pub struct EventEmitter {
    listeners: HashMap<String, Vec<EventCallback>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn on<F>(&mut self, event_name: &str, listener: F)
    where
        F: FnMut(&[Box<dyn std::any::Any>]) + 'static,
    {
        self.listeners
            .entry(event_name.to_string())
            .or_insert(Vec::new())
            .push(Box::new(listener));
    }

    pub fn emit(&mut self, event_name: &str, args: &[Box<dyn std::any::Any>]) {
        if let Some(listeners) = self.listeners.get_mut(event_name) {
            for listener in listeners {
                listener(args);
            }
        }
    }
}
