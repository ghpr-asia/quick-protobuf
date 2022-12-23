// DO NOT link to module tree
// This file is run remotely from test_deprecated_enum.rs and should fail to compile

#[path = "./test_deprecated_enum_without_add_deprecated_fields.rs"]
pub mod proto;
use crate::proto::Thing;

fn main() {
    let t = Thing::C;
}
