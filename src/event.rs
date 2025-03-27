use std::{collections::HashMap, future::Future, pin::Pin};

use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Structure of event returned from freeswitch
pub struct Event {
    pub(crate) headers: HashMap<String, Value>,
    pub(crate) body: Option<String>,
}
impl Event {
    /// Returns header from event
    pub fn headers(&self) -> &HashMap<String, Value> {
        &self.headers
    }
    /// Returns body from event
    pub fn body(&self) -> &Option<String> {
        &self.body
    }
}


/// Define the type of the event handler (callback) signature
pub type EventHandler = Box<dyn Fn(&HashMap<String, Value>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Struct
pub struct EventManager {
    handlers: HashMap<String, Vec<EventHandler>>, // Event to callback handlers map
}

impl EventManager {
    /// Create a new event manager
    pub fn new() -> Self {
        EventManager {
            handlers: HashMap::new(),
        }
    }

    /// Register an event handler (callback) for a specific event
    pub fn register_handler(&mut self, event: String, handler: EventHandler) {
        self.handlers
            .entry(event)
            .or_insert_with(Vec::new)
            .push(handler);
    }

    /// Trigger registered listeners
    pub async fn trigger_event(&self, event: String, body: &HashMap<String, Value>) {
        if let Some(handlers) = self.handlers.get(&event) {
            for handler in handlers {
                handler(body).await;
            }
        }
    }
}
