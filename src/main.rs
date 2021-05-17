mod my_thing;

use actix_rt;
use crate::my_thing::MyThing;
use std::sync::{Arc, RwLock};
use webthing::{Thing, ThingsType, WebThingServer};

use crate::my_thing::ActionMapper;

pub type DynThingRef = Arc<RwLock<Box<dyn Thing + 'static>>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let thing = MyThing::new();

    // If adding more than one thing, use ThingsType::Multiple() with a name.
    // In the single thing case, the thing's name will be broadcast.
    let mut server = WebThingServer::new(
        ThingsType::Single(thing),
        Some(8888),
        None,
        None,
        Box::new(ActionMapper),
        None,
        None,
    );
    server.start(None).await
}
