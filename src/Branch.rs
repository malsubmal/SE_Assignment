pub mod branch {
use std::collections::HashMap;
use crate::entity::entity::Queryable as Queryable;

    pub struct Branch {
        name : String,
        location : String,
    }

    impl Branch {
        pub fn new(
            name : String,
            location : String) -> Branch {
                Branch {
                    name,
                    location,
                }
        }
    }

    impl Queryable for Branch {
        fn getqueryfield(&self) -> &String {
            &self.location
        }
    }


}