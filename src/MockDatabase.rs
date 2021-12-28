pub mod mockdb {

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

    pub trait MockDatabase {
        type ID;
        type Entity;
        fn query(&self, id : Self::ID) -> Option<&Self::Entity>;
        fn shouldExist(&self, id : Self::ID) -> Result<(), String> {
            match self.query(id) {
                Some(_) => Ok(()),
                None => Err(String::from("not exist yet")),
            }
        }
        fn shouldNotExist(&self, id : Self::ID)  -> Result<(), String> {
            match self.query(id) {
                Some(_) =>  Err(String::from("already exists")),
                None => Ok(()),
            }
        }
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String>;

    }

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
    }

    impl MockDatabase for CustomerDatabase {
        type ID = String;
        type Entity = Customer;
        fn query(&self, id : Self::ID) -> Option<&Self::Entity> {
            self.db.get(&id)
        }
    
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String> {
            self.db.insert(entity.getqueryfield().to_string(), entity);
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
    }

    impl MockDatabase for CarDatabase {
        type ID = String;
        type Entity = Car;
        fn query(&self, id : Self::ID) -> Option<&Self::Entity> {
            self.db.get(&id)
        }
    
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String> {
            self.db.insert(entity.getqueryfield().to_string(), entity);
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
    }
    
    impl MockDatabase for BranchDatabase {
        type ID = String;
        type Entity = Branch;
        fn query(&self, id : Self::ID) -> Option<&Self::Entity> {
            self.db.get(&id)
        }
    
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String> {
            self.db.insert(entity.getqueryfield().to_string(), entity);
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
    }
    impl MockDatabase for ModelDatabase {
        type ID = String;
        type Entity = Model;
        fn query(&self, id : Self::ID) -> Option<&Self::Entity> {
            self.db.get(&id)
        }
    
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String> {
            self.db.insert(entity.getqueryfield().to_string(), entity);
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
    
    }
    impl MockDatabase for RentalGroupDatabase {
        type ID = String;
        type Entity = RentalGroup;
        fn query(&self, id : Self::ID) -> Option<&Self::Entity> {
            self.db.get(&id)
        }
    
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String> {
            self.db.insert(entity.getqueryfield().to_string(), entity);
            Ok(())
        } 
    }
    

    //MOCK  BRANCH NEIGHBOR DATABASE
    pub struct BranchNeighborDatabase {
        db : HashSet<(String, String)>,
    }

    impl MockDatabase for BranchNeighborDatabase {
        type ID = (String, String);
        type Entity = (String, String);

        fn query(&self, id : Self::ID) -> Option<&Self::Entity> {
            let (b1, b2) = id;
            match self.db.get(&(b1.clone(),b2.clone())) {
                None => self.db.get(&(b2,b1)),
                Some(val) => Some(val)
            }  
        }
    
        fn insert(&mut self, entity : Self::Entity) -> Result<(), String> {
            self.db.insert(entity);
            Ok(())
        } 
    }
    
    impl BranchNeighborDatabase {
    
        pub fn getSampleDB() -> BranchNeighborDatabase {
            let mut db = HashSet::new();
            //EMPTY BC HAHA HEHE
            BranchNeighborDatabase {
                db,
            }
        }
    }





}