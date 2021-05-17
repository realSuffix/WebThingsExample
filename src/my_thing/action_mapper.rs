use super::ActionMapper;
use std::sync::{RwLock, Weak};
use webthing::{server::ActionGenerator, Action, Thing};

impl ActionGenerator for ActionMapper {
    fn generate(
        &self,
        thing: Weak<RwLock<Box<dyn Thing>>>,
        name: String,
        input: Option<&serde_json::Value>,
    ) -> Option<Box<dyn Action>> {
        None
    }
}
