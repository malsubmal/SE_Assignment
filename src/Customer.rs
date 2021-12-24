pub mod customer {
use std::collections::HashMap;
use crate::entity::entity::Creatable as Creatable;
use crate::entity::entity::Queryable as Queryable;
pub struct Customer {
customername : String,
phonenumber : String,
}

impl Customer {
    pub fn new(
        customername : String,
        phonenumber : String) -> Customer {
        Customer {
                customername,
                phonenumber,
            }
    }
}

impl Queryable for Customer {
    fn getqueryfield(&self) -> &String {
        &self.phonenumber
    }
}

impl Creatable for Customer {

}




}