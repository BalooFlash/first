// use super::super::traits::get::Get;
// use super::super::traits::delete::Delete;
// use super::super::traits::edit::Edit;

// impl Get for Done {}
// impl Delete for Done {}
// impl Edit for Done {}

use super::base::Base;
use super::super::enums::TaskStatus;

#[derive(Debug)]
pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        return Done{super_struct: base}
    }
}