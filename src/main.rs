use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{CallInfo, ReturnValue, Signature};
use serde::Serialize;

// This struct represents a single row on your output stream. Fields of this
// struct will be converted to columns of the output table.
#[derive(Serialize)]
struct OutputRow {
    a: &'static str,
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

    fn do_begin_filter(&mut self, call_info: &CallInfo) -> Result<Vec<OutputRow>, anyhow::Error> {
        Ok(vec![
            OutputRow {
                a: "This is an example output row",
                b: 3,
                c: 99.999,
            },
            OutputRow {
                a: "This is another example output row",
                b: 99431,
                c: 0.3,
            },
        ])
    }
}

impl Plugin for {{ plugin_struct }} {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("{{ plugin_name }}").filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        let rows = self.do_begin_filter(&call_info).map_err(|e| {
            ShellError::labeled_error(
                format!("{{ plugin_name }} failed: {}", e),
                "error",
                &call_info.name_tag,
            )
        })?;

        let values = serde_nu::to_success_return_values(rows, &call_info.name_tag).map_err(|e| {
            ShellError::labeled_error(
                format!("failed to convert output values: {}", e),
                "failed to convert output values",
                &call_info.name_tag,
            )
        })?;

        Ok(values)
    }
}

fn main() {
    serve_plugin(&mut {{ plugin_struct }}::new());
}
