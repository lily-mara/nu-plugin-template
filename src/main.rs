use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, UntaggedValue, Value,
};

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
        Ok(vec![ReturnSuccess::value(self.len(input)?)])
    }
}

fn main() {
    serve_plugin(&mut {{ plugin_struct }}::new());
}
