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


/*

    pub fn uc_createBranch(branch : Branch) ->  Result<(), String> {
        let createBranch= Usecase::new(
            &|| branch.shouldnotexist(),
            &|| branch.addEntity(),
            &|| branch.shouldexist()
        ).run();
        createBranch
    }

    pub fn uc_createRentalGroup(rgroup : RentalGroup) ->  Result<(), String> {
        let createRGroup = Usecase::new(
            &|| rgroup.shouldnotexist(),
            &|| rgroup.addEntity(),
            &|| rgroup.shouldexist()
        ).run();
        createRGroup
    }

    pub fn uc_createCar(car : Car) ->  Result<(), String> {
        let createCar= Usecase::new(
            &|| car.shouldnotexist(),
            &|| car.addEntity(),
            &|| car.shouldexist()
        ).run();
        createCar
    }

    pub fn uc_createModel(model : Model) ->  Result<(), String> {
        let createModel= Usecase::new(
            &|| model.shouldnotexist(),
            &|| model.addEntity(),
            &|| model.shouldexist()
        ).run();
        createModel
    } */




    
}