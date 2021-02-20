use crate::model::Condition;
use serde::{Deserialize, Serialize};
use twilight_model::gateway::event::EventType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub triggers: Vec<EventType>,
    pub conditions: Condition,
}
