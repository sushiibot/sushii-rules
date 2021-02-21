use crate::model::{Condition, Context};
use serde::{Deserialize, Serialize};
use std::error::Error;
use twilight_model::gateway::event::{DispatchEvent, EventType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub trigger: EventType,
    pub conditions: Condition,
}

impl Rule {
    pub async fn check_event(
        &self,
        event: &DispatchEvent,
        context: &Context,
    ) -> Option<Result<bool, Box<dyn Error>>> {
        if event.kind() != self.trigger {
            return None;
        }

        Some(self.conditions.check_event(event, context).await)
    }
}
