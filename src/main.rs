use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, UntaggedValue, Value,
};
use serde::Serialize;

// This struct represents a single row on your output stream. Fields of this
// struct will be converted to columns of the output table.
#[derive(Serialize)]
struct OutputRow {
    a: String
    b: i32,
    c: f64,
}

struct {{ plugin_struct }} {
}

impl {{ plugin_struct }} {
    fn new() -> Self {
        Self {
        }
    }
}

impl Plugin for {{ plugin_struct }} {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("{{ plugin_name }}").filter())
    }

    fn begin_filter(&mut self, _: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(serde_nu::to_success_return_values(
            vec![OutputRow{
                a: "This is an example output row",
                b: 3,
                c: 99.999,
            }],
            &input.tag,
        ))
    }
}

fn main() {
    serve_plugin(&mut {{ plugin_struct }}::new());
}
