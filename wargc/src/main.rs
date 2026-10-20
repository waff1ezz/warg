use crate::wargc_errors::error::Error;
use wargc_driver::{Project, Flags};

mod wargc_errors;
mod wargc_driver;
mod wargc_parse;

fn main() {
    let project = Project::resolve()
        .unwrap_or_else(|err| err.throw());
    let flags = Flags::from_args()
        .unwrap_or_else(|err| err.throw());

    project.build(flags)
        .unwrap_or_else(|err| err.throw());
}
