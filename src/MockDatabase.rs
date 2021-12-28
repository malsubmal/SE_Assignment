pub mod mockdb {

    pub struct SampleSystemDatabase {

    }

    use std::collections::HashMap;
    use std::collections::HashSet;
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::model::model::ModelType as ModelType;
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
    pub struct CarDatabase {
        pub db : HashMap<String, Car>,
    }
    
    impl CarDatabase {

        pub fn getSampleDB() -> CarDatabase {
            let mut db = HashMap::new();
            let c1 = Car::new(String::from("carA"), String::from("branchA"), String::from("modelA"));
            let c2 = Car::new(String::from("carB"), String::from("branchA"), String::from("modelB"));
            db.insert(c1.getqueryfield().to_string(),c1);
            db.insert(c2.getqueryfield().to_string(),c2);
            CarDatabase {
                db,
            }
        }
    
        pub fn query(&self, id : String) -> Option<&Car> {
            self.db.get(&id)
        }
    
        pub fn insert(&mut self, car : Car) -> Result<(), String> {
            self.db.insert(car.getqueryfield().to_string(), car);
            Ok(())
        } 
    }

    //MOCK BRANCH DATABASE
    pub struct BranchDatabase {
        db : HashMap<String, Branch>,
    }
    
    impl BranchDatabase {

        pub fn getSampleDB() -> BranchDatabase {
            let mut db = HashMap::new();
            let c1 = Branch::new(String::from("A"), String::from("branchA"));
            let c2 = Branch::new(String::from("B"), String::from("branchB"));
            db.insert(c1.getqueryfield().to_string(),c1);
            db.insert(c2.getqueryfield().to_string(),c2);
            BranchDatabase {
                db,
            }
        }
    
        pub fn query(&self, id : String) -> Option<&Branch> {
            self.db.get(&id)
        }
    
        pub fn insert(&mut self, branch : Branch) -> Result<(), String> {
            self.db.insert(branch.getqueryfield().to_string(), branch);
            Ok(())
        } 
    }

    //MOCK MODEL DATABASE
    pub struct ModelDatabase {
        db : HashMap<String, Model>,
    }
    
    impl ModelDatabase {

        pub fn getSampleDB() -> ModelDatabase {
            let mut db = HashMap::new();
            let c1 = Model::new(String::from("modelA"),String::from("A"));
            let c2 = Model::new(String::from("modelB"),String::from("B"));
            db.insert(c1.getqueryfield().to_string(),c1);
            db.insert(c2.getqueryfield().to_string(),c2);
            ModelDatabase {
                db,
            }
        }
    
        pub fn query(&self, id : String) -> Option<&Model> {
            self.db.get(&id)
        }
    
        pub fn insert(&mut self, model : Model) -> Result<(), String> {
            self.db.insert(model.getqueryfield().to_string(), model);
            Ok(())
        } 
    }


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

    //MOCK  BRANCH NEIGHBOR DATABASE
    pub struct BranchNeighborDatabase {
        db : HashSet<(String, String)>,
    }
    
    impl BranchNeighborDatabase {
    
        pub fn getSampleDB() -> BranchNeighborDatabase {
            let mut db = HashSet::new();
            //EMPTY BC HAHA HEHE
/*             let c1 = (String::from("branchA"), String::from("branchB"));
            db.insert(c1); */
            BranchNeighborDatabase {
                db,
            }
        }
    
        pub fn query(&self, (b1, b2) : (String, String)) -> Option<&(String, String)> {
            match self.db.get(&(b1.clone(),b2.clone())) {
                None => self.db.get(&(b2,b1)),
                Some(val) => Some(val)
            }  
        }
    
        pub fn insert(&mut self, (b1, b2) :  (String, String)) -> Result<(), String> {
            self.db.insert((b1, b2));
            Ok(())
        } 
    }





}