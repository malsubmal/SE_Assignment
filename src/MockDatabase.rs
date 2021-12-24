pub mod mockdb {

    use std::collections::HashMap;
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;

    pub enum Errors {
        POSTCONDITIONFAIL,
        ALREADYEXIST,
        DBERROR,
    }

    //MOCK CUSTOMER DATABASE

    pub struct CustomerDatabase {
        db : HashMap<String, Customer>,
    }
    
    impl CustomerDatabase {
    
        pub fn getSampleDB() -> CustomerDatabase {
            let mut db = HashMap::new();
            let c1 = Customer::new(String::from("A"), String::from("012"));
            let c2 = Customer::new(String::from("A"), String::from("013"));
            db.insert(c1.getqueryfield().to_string(),c1);
            db.insert(c2.getqueryfield().to_string(),c2);
            CustomerDatabase {
                db,
            }
        }
    
        pub fn query(&self, id : String) -> Option<&Customer> {
            self.db.get(&id)
        }
    
        pub fn insert(&mut self, customer : Customer) -> Result<(), String> {
            self.db.insert(customer.getqueryfield().to_string(), customer);
            Ok(())
        } 
    }

    //MOCK CAR DATABASE

    //MOCK BRANCH DATABASE

    //MOCK MODEL DATABASE

    //MOCK RENTALGROUP DATABASE
    
    pub struct RentalGroupDatabase {
        db : HashMap<String, RentalGroup>,
    }
    
    impl RentalGroupDatabase {
    
        pub fn getSampleDB() -> RentalGroupDatabase {
            let mut db = HashMap::new();
            let c1 = RentalGroup::new(String::from("A"), String::from("012"));
            let c2 = RentalGroup::new(String::from("B"), String::from("013"));
            db.insert(c1.getqueryfield().to_string(),c1);
            db.insert(c2.getqueryfield().to_string(),c2);
            RentalGroupDatabase {
                db,
            }
        }
    
        pub fn query(&self, id : String) -> Option<&RentalGroup> {
            self.db.get(&id)
        }
    
        pub fn insert(&mut self, rentalGroup : RentalGroup) -> Result<(), String> {
            self.db.insert(rentalGroup.getqueryfield().to_string(), rentalGroup);
            Ok(())
        } 
    }





}