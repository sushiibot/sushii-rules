use crate::model::Status;

pub struct ConditionResult {
    name: String,
    status: Status,
    children: Vec<ConditionResult>,
}
