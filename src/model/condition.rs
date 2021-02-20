use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use std::error::Error;
use twilight_model::gateway::event::DispatchEvent;

use crate::model::{ConditionResult, Constraint, Context, Status};

#[derive(Debug, Serialize, Deserialize)]
pub enum Condition {
    And {
        and: Vec<Condition>,
    },
    Or {
        or: Vec<Condition>,
    },
    Not {
        not: Box<Condition>,
    },
    AtLeast {
        min_count: usize,
        conditions: Vec<Condition>,
    },
    Condition {
        #[serde(flatten)]
        constraint: Constraint,
    },
}

impl Condition {
    #[async_recursion]
    async fn check_event(
        &self,
        event: &DispatchEvent,
        context: &Context,
    ) -> Result<bool, Box<dyn Error>> {
        match *self {
            Condition::And { ref and } => {
                for child in and.iter() {
                    if !child.check_event(event, context).await? {
                        return Ok(false);
                    }
                }

                return Ok(true);
            }
            Condition::Not { not: ref c } => {
                return c.check_event(event, context).await.map(|r| !r);
            }
            Condition::Or { ref or } => {
                for child in or.iter() {
                    if child.check_event(event, context).await? {
                        return Ok(true);
                    }
                }

                return Ok(false);
            }
            Condition::AtLeast {
                min_count,
                ref conditions,
            } => {
                let mut count = 0;

                for child in conditions.iter() {
                    if child.check_event(event, context).await? {
                        count += 1;
                    }
                }

                return Ok(count >= min_count);
            }
            Condition::Condition { ref constraint } => constraint.check_event(event).await,
        }
    }
}