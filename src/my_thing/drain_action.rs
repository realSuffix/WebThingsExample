use std::{sync::RwLock, sync::Weak, thread, time};

use serde_json::{json, Value};
use uuid::Uuid;
use webthing::{Action, BaseAction, BaseEvent, Thing};

use crate::DynThingRef;

use super::{DrainAction, MyThing};

impl DrainAction {
    pub const DRAIN_ACTION_NAME: &'static str = "drain";

    pub fn new(
        input: Option<serde_json::Map<String, serde_json::Value>>,
        thing: Weak<RwLock<Box<dyn Thing>>>,
    ) -> DrainAction {
        DrainAction(BaseAction::new(
            Uuid::new_v4().to_string(),
            Self::DRAIN_ACTION_NAME.to_owned(),
            input,
            thing,
        ))
    }
}

impl Action for DrainAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<DynThingRef> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();

        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let input = self.get_input().unwrap().clone();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            thread::sleep(time::Duration::from_secs(
                input.get("duration").unwrap().as_u64().unwrap(),
            ));

            let thing = thing.clone();
            let mut thing = thing.write().unwrap();

            if let Some(value) = input.get("drain").map(Value::as_u64).flatten() {
                let _ = thing.set_property(
                    MyThing::WATER_PROP_NAME.to_owned(),
                    input.get("drain").unwrap().clone(),
                );

                if value < 250 {
                    thing.add_event(Box::new(BaseEvent::new(
                        "almostEmpty".to_owned(),
                        Some(json!(value)),
                    )))
                }
            }

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}
