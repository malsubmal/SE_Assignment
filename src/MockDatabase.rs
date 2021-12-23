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

    pub struct CustomerDatabase {
        db : HashMap<String, Customer>,
    }
    
    impl CustomerDatabase {
    
        fn getSampleDB() -> CustomerDatabase {
            let mut db = HashMap::new();
            let c1 = Customer::new(String::from("A"), String::from("012"));
            let c2 = Customer::new(String::from("A"), String::from("013"));
            db.insert(c1.getqueryfield().to_string(),c1);
            db.insert(c2.getqueryfield().to_string(),c2);
            CustomerDatabase {
                db,
            }
        }
    
        fn queryCustomer(&self, id : String) -> Option<&Customer> {
            self.db.get(&id)
        }

        pub fn CustomerShouldExist(&self, id : String) ->  Result<(), String> {
            match self.queryCustomer(id) {
                Some(_) => Ok(()),
                None => Err(String::from("not exist yet")),
            }
        }
        pub fn CustomerShouldNotExist(&self, id : String) -> Result<(), String> {
            match self.queryCustomer(id) {
                Some(_) =>  Err(String::from("already exists")),
                None => Ok(()),
            }
        }
    
        pub fn insertCustomer(&mut self, customer : Customer) -> Result<(), String> {
            self.db.insert(customer.getqueryfield().to_string(), customer);
            Ok(())
        } 
    }


}