pub mod usescases {
    
    use crate::usecase::usecase::Usecase as Usecase;
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::mockdatabase::mockdb::CustomerDatabase as CustomerDB;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;

    
    pub fn uc_createCustomer(customer : Customer) -> Result<(), String> {
        let createCustomer = Usecase::new(
            &|| customer.shouldnotexist(),
            &|| customer.addEntity(),
            &|| customer.shouldexist()
        ).run();
        createCustomer
    }

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
    }
    
}