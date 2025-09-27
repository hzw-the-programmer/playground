use tracing::{debug_span, info, info_span};
use tracing_subscriber::prelude::*;

mod custom_layer;
use custom_layer::CustomLayer;

fn main() {
    // Set up how `tracing-subscriber` will deal with tracing data.
    tracing_subscriber::registry().with(CustomLayer).init();

    let outer_span = info_span!("outer", level = 0, other_field = tracing::field::Empty);
    let _outer_entered = outer_span.enter();

    let inner_span = debug_span!("inner", level = 1);
    let _inner_entered = inner_span.enter();

    outer_span.record("other_field", &7);

    // Log something simple. In `tracing` parlance, this creates an "event".
    info!(a_bool = true, answer = 42, message = "first example");
}
