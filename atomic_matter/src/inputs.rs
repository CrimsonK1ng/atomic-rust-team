use serde::{Deserialize, Serialize};
/*
Example inputs block for given AtomicTest
    output_file:
      description: Path where resulting dump should be placed
      type: path
      default: C:\Windows\Temp\lsass_dump.dmp
*/
/// Contains a single input with its description, default value, and optionally contains a type
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Inputs {
    pub description: String,
    // https://stackoverflow.com/questions/69871539/is-it-possible-to-define-a-field-use-the-keywords-in-rust
    #[serde(rename = "type")]
    pub atomictype: String,
    pub default: String,
}
