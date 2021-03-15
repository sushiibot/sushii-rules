use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::error::Error;
use twilight_model::gateway::event::{DispatchEvent};

use crate::model::{Condition, Context};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Rule {
    // pub trigger: EventType,
    pub conditions: Condition,
}

impl Rule {
    pub async fn check_event(
        &self,
        event: &DispatchEvent,
        context: &Context,
    ) -> Option<Result<bool, Box<dyn Error>>> {
        // if event.kind() != self.trigger {
        //     return None;
        // }

        Some(self.conditions.check_event(event, context).await)
    }
}
