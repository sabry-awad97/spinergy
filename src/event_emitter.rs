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

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn test_event_emitter() {
        // Create a new EventEmitter
        let mut emitter = EventEmitter::new();

        // Define a counter for tracking the number of times the event is emitted
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        // Define a listener function
        let listener = move |args: &[Box<dyn std::any::Any>]| {
            *counter_clone.lock().unwrap() += 1;
            println!("Event emitted with arguments:");
            for arg in args {
                if let Some(value) = arg.downcast_ref::<i32>() {
                    println!("i32: {}", value);
                } else if let Some(value) = arg.downcast_ref::<&str>() {
                    println!("&str: {}", value);
                } else if let Some(value) = arg.downcast_ref::<f64>() {
                    println!("f64: {}", value);
                } else {
                    println!("Unknown type");
                }
            }
        };

        // Register the listener for the "my_event" event
        emitter.on("my_event", listener);

        // Emit the "my_event" event with some arguments
        let args: Vec<Box<dyn std::any::Any>> =
            vec![Box::new(42), Box::new("Hello"), Box::new(3.14)];
        emitter.emit("my_event", &args);

        // Verify that the listener was invoked and the counter was incremented
        assert_eq!(*counter.lock().unwrap(), 1);
    }
}
