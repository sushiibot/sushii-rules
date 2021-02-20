use serde::{Deserialize, Serialize};
use std::error::Error;
use twilight_model::gateway::event::DispatchEvent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "operator", content = "value")]
pub enum MessageConstraint {
    ContentEquals(String),
    ContentNotEquals(String),
    ContentContains(String),
    ContentContainsAll(Vec<String>),
    ContentContainsAny(Vec<String>),
    ContentDoesNotContain(String),
    ContentDoesNotContainAny(Vec<String>),
    ContentIn(Vec<String>),
    ContentNotIn(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "event_type", content = "value")]
pub enum Constraint {
    Message(MessageConstraint),
}

impl Constraint {
    pub async fn check_event(&self, event: &DispatchEvent) -> Result<bool, Box<dyn Error>> {
        let val = match (self, event) {
            #[rustfmt::skip]
            (Constraint::Message(c), DispatchEvent::MessageCreate(msg)) => {
                match c {
                    MessageConstraint::ContentEquals(s) => {
                        msg.content == *s
                    }
                    MessageConstraint::ContentNotEquals(s) => {
                        msg.content != *s
                    }
                    MessageConstraint::ContentContains(s) => {
                        msg.content.contains(s)
                    }
                    MessageConstraint::ContentContainsAll(strs) => {
                        strs.iter().all(|s| msg.content.contains(s))
                    },
                    MessageConstraint::ContentContainsAny(strs) => {
                        strs.iter().any(|s| msg.content.contains(s))
                    },
                    MessageConstraint::ContentDoesNotContain(s) => {
                        !msg.content.contains(s)
                    },
                    MessageConstraint::ContentDoesNotContainAny(strs) => {
                        !strs.iter().all(|s| msg.content.contains(s))
                    },
                    MessageConstraint::ContentIn(strs) => {
                        strs.iter().all(|s| s.contains(&msg.content))
                    }
                    MessageConstraint::ContentNotIn(strs) => {
                        !strs.iter().all(|s| s.contains(&msg.content))
                    }
                }
            },
            _ => unimplemented!(),
        };

        Ok(val)
    }
}
