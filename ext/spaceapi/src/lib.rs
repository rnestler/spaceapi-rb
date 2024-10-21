use magnus::{exception, function, prelude::*, Error, Ruby, Value};
use serde_json;
use serde_magnus;

use spaceapi::Status;

fn status_from_string(json_string: String) -> Result<Value, Error> {
    let status: Status = serde_json::from_str(&json_string)
        .map_err(|err| Error::new(exception::runtime_error(), err.to_string()))?;
    serde_magnus::serialize(&status)
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("Spaceapi")?;
    module.define_singleton_method("status_from_string", function!(status_from_string, 1))?;
    Ok(())
}
