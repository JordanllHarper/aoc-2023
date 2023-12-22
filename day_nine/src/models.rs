#[derive(Debug, Clone)]
pub struct ListStage {
    pub first_of_this_stage: i64,
    pub previous_step: Box<Option<ListStage>>,
}

impl ListStage {
    pub fn new(first_of_this_stage: i64, previous_step: Box<Option<ListStage>>) -> Self {
        Self {
            first_of_this_stage,
            previous_step,
        }
    }
}
