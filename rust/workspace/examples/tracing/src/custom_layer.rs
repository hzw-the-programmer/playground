use tracing_subscriber::Layer;

pub struct CustomLayer;

impl<S> Layer<S> for CustomLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        // What's the parent span look like?
        let parent_span = ctx.event_span(event).unwrap();
        println!("parent span");
        println!("  name={}", parent_span.name());
        println!("  target={}", parent_span.metadata().target());

        println!();

        // What's about all of the spans that are in scope?
        let scope = ctx.event_scope(event).unwrap();
        for span in scope.from_root() {
            println!("an ancestor span");
            println!("  name={}", span.name());
            println!("  target={}", span.metadata().target());
        }
    }

        fn new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(id).unwrap();
        println!("Got new_span!");
        println!("  level={:?}", span.metadata().level());
        println!("  target={:?}", span.metadata().target());
        println!("  name={:?}", span.metadata().name());

        // Our old friend, `println!` exploration.
        let mut visitor = PrintlnVisitor;
        attrs.record(&mut visitor);
    }
}

struct PrintlnVisitor;

impl tracing::field::Visit for PrintlnVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        println!("  field={} value={}", field.name(), value)
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        println!("  field={} value={:?}", field.name(), value)
    }
}

// use std::collections::BTreeMap;

// struct JsonVisitor<'a>(&'a mut BTreeMap<String, serde_json::Value>);

// impl<'a> tracing::field::Visit for JsonVisitor<'a> {
//     fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!(value));
//     }

//     fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!(value));
//     }

//     fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!(value));
//     }

//     fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!(value));
//     }

//     fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!(value));
//     }

//     fn record_error(
//         &mut self,
//         field: &tracing::field::Field,
//         _value: &(dyn std::error::Error + 'static),
//     ) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!("record_error"));
//     }

//     fn record_debug(&mut self, field: &tracing::field::Field, _value: &dyn std::fmt::Debug) {
//         self.0
//             .insert(field.name().to_string(), serde_json::json!("record_debug"));
//     }
// }
