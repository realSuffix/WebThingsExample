use super::{ActionMapper, DrainAction};
use std::sync::{RwLock, Weak};
use webthing::{server::ActionGenerator, Action, Thing};

impl ActionGenerator for ActionMapper {
    fn generate(
        &self,
        thing: Weak<RwLock<Box<dyn Thing>>>,
        name: String,
        input: Option<&serde_json::Value>,
    ) -> Option<Box<dyn Action>> {
        input
            .map(Clone::clone)
            .map(|input| match &name[..] {
                DrainAction::DRAIN_ACTION_NAME => Some(Box::new(DrainAction::new(
                    input.as_object().cloned(),
                    thing,
                )) as Box<dyn Action>),
                _ => None,
            })
            .flatten()
    }
}
