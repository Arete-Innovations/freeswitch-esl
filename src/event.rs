use std::{collections::HashMap, fmt, future::Future, pin::Pin};

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
pub type EventHandlerExecution = Box<dyn Fn(&HashMap<String, Value>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// A wrapper type for EventHandler that implements Debug
pub struct EventHandler{
    execute: EventHandlerExecution
}

/// Event handler wrapper for execution
impl EventHandler {
    /// Factory to create new event handler
    pub fn new(execute :EventHandlerExecution) -> Self {
        Self {execute: execute}
    }
}
impl fmt::Debug for EventHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We can't directly print the function, so print a placeholder.
        f.debug_struct("EventHandler")
            .field("handler", &"Function (cannot display)")
            .finish()
    }
}


/// Struct
#[derive(Debug)]
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
                ((handler).execute)(body).await;
            }
        }
    }
}
