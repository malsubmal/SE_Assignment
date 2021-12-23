pub mod test {
    use crate::usecase::usecase::Usecase as Usecase;
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;
    use crate::usecases::usescases as UC;

  #[test]
    fn test_uc_createCustomer(){
        let customer = Customer::new(String::from("V"), String::from("existing"));
        let res = UC::uc_createCustomer(customer);
        assert_eq!(res.unwrap_err(), String::from("already exists"));
    } 

    #[test]
    fn test_uc_createCustomer_connErr(){
        let customer = Customer::new(String::from("V"), String::from("problem"));
        let res = UC::uc_createCustomer(customer);
        assert_eq!(res.unwrap_err(), String::from("problem querying database"));
    } 

    #[test]
    fn test_uc_createCustomer_success(){
        let customer = Customer::new(String::from("V"), String::from("012"));
        let res = UC::uc_createCustomer(customer);
        assert_eq!(res.unwrap(), ());
    } 

}