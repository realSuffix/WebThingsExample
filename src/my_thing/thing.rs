use super::MyThing;
use std::sync::{Arc, RwLock};

use serde_json::json;
use webthing::{BaseProperty, BaseThing, Thing};

use crate::{my_thing::DrainAction, DynThingRef};

impl MyThing {
    pub const WATER_PROP_NAME: &'static str = "water";
    pub fn new() -> DynThingRef {
        let mut thing = Self::create_thing();

        Self::add_water_property(&mut thing);
        Self::add_drain_action(&mut thing);
        Self::add_almost_empty_event(&mut thing);

        Arc::new(RwLock::new(Box::new(thing)))
    }

    fn create_thing() -> BaseThing {
        BaseThing::new(
            "urn:dev:ops:my-lamp-1234".to_owned(),
            "My pool".to_owned(),
            None,
            Some("A web connected lamp".to_owned()),
        )
    }

    fn add_water_property(thing: &mut dyn Thing) {
        let water_description = json!({
            "@type" : "LevelProperty",
            "title": "Water",
            "type": "number",
            "description": "The level of water from 0-1000",
            "minimum": 0,
            "maximum": 1000,
            "unit": "liter"
        });

        let water_description = water_description.as_object().unwrap().clone();
        thing.add_property(Box::new(BaseProperty::new(
            Self::WATER_PROP_NAME.to_owned(),
            json!(800),
            None,
            Some(water_description),
        )));
    }

    fn add_drain_action(thing: &mut dyn Thing) {
        let drain_metadata = json!({
            "title": "Drain",
            "description": "Drain the pool by a given amount over a specific amount of time",
            "input": {
                "type": "object",
                "required": [
                    "drain",
                    "duration"
                ],
                "properties": {
                    "drain": {
                        "type": "integer",
                        "minimum": 0,
                        "maximum": 1000,
                        "unit": "liter"
                    },
                    "duration": {
                        "type": "integer",
                        "minimum": 1,
                        "unit": "seconds"
                    }
                }
            }
        });

        let drain_metadata = drain_metadata.as_object().unwrap().clone();
        thing.add_available_action(DrainAction::DRAIN_ACTION_NAME.to_owned(), drain_metadata);
    }

    fn add_almost_empty_event(thing: &mut dyn Thing) {
        let almost_empty_metadata = json!({
            "description": "The pool is almost empty (less than 1/4 left!).",
            "type": "number",
            "unit": "liter"
        });
        let almost_empty_metadata = almost_empty_metadata.as_object().unwrap().clone();
        thing.add_available_event("almostEmpty".to_owned(), almost_empty_metadata);
    }
}
