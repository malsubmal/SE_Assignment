pub mod usescases {
    use crate::customer::customer::Customer as Customer;
    use std::ops::Deref;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::mockdatabase::mockdb::MockDatabase;
    use crate::carreturn::carreturn::CarReturn as CarReturn;
    use crate::car::car::RentalStatus as RentalStatus;
    use crate::mockdatabase::mockdb;
    use crate::entity::entity::Queryable as Queryable;

    pub fn uc_recordCarReturn(
        car : String,
        db_car : &mut mockdb::CarDatabase,
        daytime : String
    ) -> Result<(), String> {
        let car_id = car.clone();
        //pre-condition
        db_car.shouldExist(car.clone())?;
        //db_customer.shouldExist(customer.clone())?;
        //maybe also check daytime not sure 
        //let cr = CarReturn::new(car.clone(), customer, daytime).record()?;
        db_car.queryMut(car).unwrap().returnCar();
        //post-condition car status is now RETURNED
        match db_car.query(car_id).unwrap().getRentalStatus() {
            RentalStatus::RETURNED => Ok(()),
            _ => Err(String::from("Error checking car rental status"))
        }?;
        Ok(())
    }

    pub fn uc_addBranchNeighbor(b1: String, b2: String, db_branch : &mockdb::BranchDatabase, 
    db_neighbor : &mut mockdb::BranchNeighborDatabase)
    -> Result<(), String> {
        //pre-condition : both branches exist
        db_branch.shouldExist(b1.clone())?;
        db_branch.shouldExist(b2.clone())?;
        //pre-condition : neighboring not exist
        db_neighbor.shouldNotExist((b1.clone(),b2.clone()))?;
        //action
        db_neighbor.insert((b1.clone(),b2.clone()))?;
        //post-condition
        db_neighbor.shouldExist((b1.clone(),b2.clone()))?;
        Ok(())

    }

    pub fn uc_createCustomer(db : &mut mockdb::CustomerDatabase, customer : Customer) -> Result<(), String> {
        let customer_id = customer.getqueryfield().to_string();
        //pre-condition
        db.shouldNotExist(customer_id.clone())?;
        //action
        db.insert(customer)?;
        //post-condition
        db.shouldExist(customer_id.clone())?;
        Ok(())
    }

    pub fn uc_createRentalGroup(db : &mut mockdb::RentalGroupDatabase, item : RentalGroup)
     -> Result<(), String> {
        let id = item.getqueryfield().to_string();
        //pre-condition
        db.shouldNotExist(id.clone())?;
        //action
        db.insert(item)?;
        //post-condition
        db.shouldExist(id.clone())?;
        Ok(())
    }

    pub fn uc_createBranch(db : &mut mockdb::BranchDatabase, item : Branch)
    -> Result<(), String> {
        let id = item.getqueryfield().to_string();
        //pre-condition
        db.shouldNotExist(id.clone())?;
        //action
        db.insert(item)?;
        //post-condition
        db.shouldExist(id.clone())?;
        Ok(())
   }

   pub fn uc_createModel(db : &mut mockdb::ModelDatabase, item : Model, db_support: &mockdb::RentalGroupDatabase)
   -> Result<(), String> {
      let id = item.getqueryfield().to_string();
      let rg = item.getrentalgroup().to_string();
      //pre-condition id is new
      db.shouldNotExist(id.clone())?;
      //pre-condition ModelGroup exists on database
      db_support.shouldExist(rg)?;
      //action
      db.insert(item)?;
      //post-condition
      db.shouldExist(id.clone())?;
      Ok(())
  }

  pub fn uc_createCar(db : &mut mockdb::CarDatabase, item : Car,
     db_branch: &mockdb::BranchDatabase, db_model: &mockdb::ModelDatabase)
  -> Result<(), String> {
     let id = item.getqueryfield().to_string();
     let model_id = item.getModel().to_string();
     let branch_id = item.getBranch().to_string();
     //pre-condition id is new
     db.shouldNotExist(id.clone())?;
     //pre-condition model exists on database
     db_model.shouldExist(model_id)?;
    //pre-condition branch exists on database
     db_branch.shouldExist(branch_id)?;
     //action
     db.insert(item)?;
     //post-condition
     db.shouldExist(id.clone())?;
     Ok(())
 }

 //I KNOW THE CODE IS SMELLY MY BRAIN IS SMALL AND MY MORTAL COIL TIRED DO NOT TALK TO ME ABOUT IT

 pub fn uc_queryCar_branch_rg(
    db : &mockdb::CarDatabase,
    db_branch: &mockdb::BranchDatabase,
    db_rentalgroup: &mockdb::RentalGroupDatabase,
    db_model: &mockdb::ModelDatabase,
    branch_id : String,
    rentalgroup_id : String) -> Result<Vec<Car>, String>  {


         //precondition branch exists
         db_branch.shouldExist(branch_id.clone())?;
        //precondition rentalgroup exists
        db_rentalgroup.shouldExist(rentalgroup_id.clone())?;

        let cars :Vec<Car> = db.db
                    .values()
                    .filter(|car| car.getBranch().to_string() == branch_id &&
                    db_model.query(car.getModel().to_string()).unwrap().getrentalgroup().to_string() == rentalgroup_id)
                    .cloned()
                    .collect();
        Ok(cars)

        //no post-condition

    }

    
}