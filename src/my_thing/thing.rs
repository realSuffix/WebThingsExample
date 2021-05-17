use super::MyThing;
use std::sync::{Arc, RwLock};

use serde_json::json;
use webthing::{BaseProperty, BaseThing, Thing};

use crate::DynThingRef;

impl MyThing {
    pub fn new() -> DynThingRef {
        let mut thing = Self::create_thing();
        Self::add_on_property(&mut thing);
        Arc::new(RwLock::new(Box::new(thing)))
    }

    fn create_thing() -> BaseThing {
        BaseThing::new(
            "urn:dev:ops:my-lamp-1234".to_owned(),
            "My Lamp FF ...".to_owned(),
            Some(vec!["OnOffSwitch".to_owned(), "Light".to_owned()]),
            Some("A web connected lamp".to_owned()),
        )
    }

    fn add_on_property(thing: &mut dyn Thing) {
        let on_description = json!({
            "title": "On/Off",
            "type": "boolean",
            "description": "Whether the lamp is turned on"
        });

        let on_description = on_description.as_object().unwrap().clone();

        thing.add_property(Box::new(BaseProperty::new(
            "on".to_owned(),
            json!(true),
            None,
            Some(on_description),
        )));
    }
}
