pub mod test {
    use crate::usecase::usecase::Usecase as Usecase;
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;

    fn uc_createCustomer(customer : Customer) -> Result<(), String> {
        let createCustomer = Usecase::new(
            &|| customer.shouldnotexist(),
            &|| customer.addEntity(),
            &|| customer.shouldexist()
        ).run();
        createCustomer
    }

    fn uc_createBranch(branch : Branch) ->  Result<(), String> {
        let createBranch= Usecase::new(
            &|| branch.shouldnotexist(),
            &|| branch.addEntity(),
            &|| branch.shouldexist()
        ).run();
        createBranch
    }

    fn uc_createRentalGroup(rgroup : RentalGroup) ->  Result<(), String> {
        let createRGroup = Usecase::new(
            &|| rgroup.shouldnotexist(),
            &|| rgroup.addEntity(),
            &|| rgroup.shouldexist()
        ).run();
        createRGroup
    }

    fn uc_createCar(car : Car) ->  Result<(), String> {
        let createCar= Usecase::new(
            &|| car.shouldnotexist(),
            &|| car.addEntity(),
            &|| car.shouldexist()
        ).run();
        createCar
    }

    fn uc_createModel(model : Model) ->  Result<(), String> {
        let createModel= Usecase::new(
            &|| model.shouldnotexist(),
            &|| model.addEntity(),
            &|| model.shouldexist()
        ).run();
        createModel
    }
    

    #[test]
    fn test_uc_createCustomer(){
        let customer = Customer::new(String::from("V"), String::from("existing"));
        let res = uc_createCustomer(customer);
        assert_eq!(res.unwrap_err(), String::from("already exists"));
    } 

    #[test]
    fn test_uc_createCustomer_connErr(){
        let customer = Customer::new(String::from("V"), String::from("problem"));
        let res = uc_createCustomer(customer);
        assert_eq!(res.unwrap_err(), String::from("problem querying database"));
    } 

    #[test]
    fn test_uc_createCustomer_success(){
        let customer = Customer::new(String::from("V"), String::from("012"));
        let res = uc_createCustomer(customer);
        assert_eq!(res.unwrap(), ());
    } 

}