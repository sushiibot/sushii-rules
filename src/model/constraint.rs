use serde::{Deserialize, Serialize};
use std::error::Error;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::Context;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "operator", content = "value")]
pub enum StringConstraint {
    Equals(String),
    NotEquals(String),
    Contains(String),
    ContainsAll(Vec<String>),
    ContainsAny(Vec<String>),
    DoesNotContain(String),
    DoesNotContainAny(Vec<String>),
    In(Vec<String>),
    NotIn(Vec<String>),
}

impl StringConstraint {
    #[rustfmt::skip]
    pub fn check_string(&self, in_str: &str) -> bool {
        match self {
            StringConstraint::Equals(s) => {
                in_str == *s
            }
            StringConstraint::NotEquals(s) => {
                in_str != *s
            }
            StringConstraint::Contains(s) => {
                in_str.contains(s)
            }
            StringConstraint::ContainsAll(strs) => {
                strs.iter().all(|s| in_str.contains(s))
            },
            StringConstraint::ContainsAny(strs) => {
                strs.iter().any(|s| in_str.contains(s))
            },
            StringConstraint::DoesNotContain(s) => {
                !in_str.contains(s)
            },
            StringConstraint::DoesNotContainAny(strs) => {
                !strs.iter().all(|s| in_str.contains(s))
            },
            StringConstraint::In(strs) => {
                strs.iter().all(|s| s.contains(&in_str))
            }
            StringConstraint::NotIn(strs) => {
                !strs.iter().all(|s| s.contains(&in_str))
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "message_field", content = "value")]
pub enum MessageConstraint {
    Content(StringConstraint),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "event_type", content = "value")]
pub enum Constraint {
    Message(MessageConstraint),
}

impl Constraint {
    pub async fn check_event(
        &self,
        event: &DispatchEvent,
        context: &Context,
    ) -> Result<bool, Box<dyn Error>> {
        let val = match (self, event) {
            (
                Constraint::Message(MessageConstraint::Content(c)),
                DispatchEvent::MessageCreate(msg),
            ) => c.check_string(&msg.content),
            _ => unimplemented!(),
        };

        Ok(val)
    }
}
