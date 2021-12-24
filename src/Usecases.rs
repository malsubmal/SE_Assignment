pub mod usescases {
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::mockdatabase::mockdb;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;




    pub fn uc_createCustomer(db : &mut mockdb::CustomerDatabase, customer : Customer) -> Result<(), String> {
        let customer_id = customer.getqueryfield().to_string();
        //pre-condition
        match db.query(customer_id.clone()) {
            Some(_) =>  Err(String::from("already exists")),
            None => Ok(()),
        }?;
        //action
        db.insert(customer)?;
        //post-condition
        match db.query(customer_id.clone()) {
            Some(_) => Ok(()),
            None => Err(String::from("not exist yet")),
        }?;
        Ok(())
    }

    pub fn uc_createRentalGroup(db : &mut mockdb::RentalGroupDatabase, item : RentalGroup)
     -> Result<(), String> {
        let id = item.getqueryfield().to_string();
        //pre-condition
        match db.query(id.clone()) {
            Some(_) =>  Err(String::from("already exists")),
            None => Ok(()),
        }?;
        //action
        db.insert(item)?;
        //post-condition
        match db.query(id.clone()) {
            Some(_) => Ok(()),
            None => Err(String::from("not exist yet")),
        }?;
        Ok(())
    }

    pub fn uc_createBranch(db : &mut mockdb::BranchDatabase, item : Branch)
    -> Result<(), String> {
       let id = item.getqueryfield().to_string();
       //pre-condition
       match db.query(id.clone()) {
           Some(_) =>  Err(String::from("already exists")),
           None => Ok(()),
       }?;
       //action
       db.insert(item)?;
       //post-condition
       match db.query(id.clone()) {
           Some(_) => Ok(()),
           None => Err(String::from("not exist yet")),
       }?;
       Ok(())
   }

   pub fn uc_createModel(db : &mut mockdb::ModelDatabase, item : Model, db_support: &mockdb::RentalGroupDatabase)
   -> Result<(), String> {
      let id = item.getqueryfield().to_string();
      let rg = item.getrentalgroup().to_string();
      //pre-condition id is new
      match db.query(id.clone()) {
          Some(_) =>  Err(String::from("already exists")),
          None => Ok(()),
      }?;
      //pre-condition ModelGroup exists on database
      match db_support.query(rg)  {
        Some(_) => Ok(()),
        None => Err(String::from("not exist yet")),
    }?;
      //action
      db.insert(item)?;
      //post-condition
      match db.query(id.clone()) {
          Some(_) => Ok(()),
          None => Err(String::from("not exist yet")),
      }?;
      Ok(())
  }

  pub fn uc_createCar(db : &mut mockdb::CarDatabase, item : Car,
     db_branch: &mockdb::BranchDatabase, db_model: &mockdb::ModelDatabase)
  -> Result<(), String> {
     let id = item.getqueryfield().to_string();
     let model_id = item.getModel().to_string();
     let branch_id = item.getBranch().to_string();
     //pre-condition id is new
     match db.query(id.clone()) {
         Some(_) =>  Err(String::from("already exists")),
         None => Ok(()),
     }?;
     //pre-condition model exists on database
     match db_model.query(model_id)  {
       Some(_) => Ok(()),
       None => Err(String::from("not exist yet")),
   }?;
    //pre-condition branch exists on database
    match db_branch.query(branch_id)  {
        Some(_) => Ok(()),
        None => Err(String::from("not exist yet")),
    }?;
     //action
     db.insert(item)?;
     //post-condition
     match db.query(id.clone()) {
         Some(_) => Ok(()),
         None => Err(String::from("not exist yet")),
     }?;
     Ok(())
 }

    
}