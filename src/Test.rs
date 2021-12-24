pub mod test {
    use crate::customer::customer::Customer as Customer;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
    use crate::branch::branch::Branch as Branch;
    use crate::model::model::Model as Model;
    use crate::car::car::Car as Car;
    use crate::mockdatabase::mockdb;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;
    use crate::usecases::usescases as uc;
    
    #[test]
    fn test_uc_createCustomer_fail() {
        //CASE: CUSTOMER ID ALREADY EXISTS
        let mut db = mockdb::CustomerDatabase::getSampleDB();
        let c2 = Customer::new(String::from("A"), String::from("013"));
        assert_eq!(uc::uc_createCustomer(&mut db, c2).is_err(),true);
    }

    #[test]
    fn test_uc_createCustomer_success() {
        //CASE: CUSTOMER CREATED SUCCESSFULLY
        let mut db = mockdb::CustomerDatabase::getSampleDB();
        let c2 = Customer::new(String::from("A"), String::from("023"));
        assert_eq!(uc::uc_createCustomer(&mut db, c2).is_err(),false);
    }

    
    #[test]
    fn test_uc_createRG_fail() {
        //CASE: RG ID ALREADY EXISTS
        let mut db = mockdb::RentalGroupDatabase::getSampleDB();
        let c1 = RentalGroup::new(String::from("A"), String::from("012"));
        assert_eq!(uc::uc_createRentalGroup(&mut db, c1).is_err(),true);
    }

    #[test]
    fn test_uc_createRG_success() {
        //CASE: RG CREATED SUCCESSFULLY
        let mut db = mockdb::RentalGroupDatabase::getSampleDB();
        let c1 = RentalGroup::new(String::from("C"), String::from("012"));
        assert_eq!(uc::uc_createRentalGroup(&mut db, c1).is_err(),false);
    }

}