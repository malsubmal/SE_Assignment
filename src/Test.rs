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

    
    #[test]
    fn test_uc_createBranch_fail() {
        //CASE: ID ALREADY EXISTS
        let mut db = mockdb::BranchDatabase::getSampleDB();
        let c1 = Branch::new(String::from("A"), String::from("branchA"));
        assert_eq!(uc::uc_createBranch(&mut db, c1).is_err(),true);
    }

    #[test]
    fn test_uc_createBranch_success() {
        //CASE: CREATED SUCCESSFULLY
        let mut db = mockdb::BranchDatabase::getSampleDB();
        let c1 = Branch::new(String::from("A"), String::from("branchC"));
        assert_eq!(uc::uc_createBranch(&mut db, c1).is_err(),false);
    }

    #[test]
    fn test_uc_createModel_fail() {
        //CASE: ID ALREADY EXISTS
        let sup_db = mockdb::RentalGroupDatabase::getSampleDB();
        let mut db = mockdb::ModelDatabase::getSampleDB();
        let c1 = Model::new(String::from("modelA"), String::from("A"));
        assert_eq!(uc::uc_createModel(&mut db, c1, &sup_db).is_err(),true);
    }

    #[test]
    fn test_uc_createModel_fail_2() {
        //CASE: RENTALGROUP DOESNT EXIST
        let  sup_db = mockdb::RentalGroupDatabase::getSampleDB();
        let mut db = mockdb::ModelDatabase::getSampleDB();
        let c1 = Model::new(String::from("modelD"), String::from("E"));
        assert_eq!(uc::uc_createModel(&mut db, c1, &sup_db).is_err(),true);
    }

    #[test]
    fn test_uc_createModel_success() {
        //CASE: CREATED SUCCESSFULLY
        let sup_db = mockdb::RentalGroupDatabase::getSampleDB();
        let mut db = mockdb::ModelDatabase::getSampleDB();
        let c1 = Model::new(String::from("modelD"), String::from("A"));
        assert_eq!(uc::uc_createModel(&mut db, c1, &sup_db).is_err(),false);
    }

    

    #[test]
    fn test_uc_createCar_fail() {
        //CASE: ID ALREADY EXISTS
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let db_model = mockdb::ModelDatabase::getSampleDB();
        let mut db = mockdb::CarDatabase::getSampleDB();
        let c1 = Car::new(String::from("carA"), String::from("branchA"), String::from("modelA"));
        assert_eq!(uc::uc_createCar(&mut db, c1, &db_branch, &db_model).is_err(),true);
    }

    #[test]
    fn test_uc_createCar_fail_2() {
        //CASE: MODEL DOESNT EXIST
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let db_model = mockdb::ModelDatabase::getSampleDB();
        let mut db = mockdb::CarDatabase::getSampleDB();
        let c1 = Car::new(String::from("carD"), String::from("branchA"), String::from("Model_NO"));
        assert_eq!(uc::uc_createCar(&mut db, c1, &db_branch, &db_model).is_err(),true);
    }

    #[test]
    fn test_uc_createCar_fail_3() {
        //CASE: BRANCH DOESNT EXIST
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let db_model = mockdb::ModelDatabase::getSampleDB();
        let mut db = mockdb::CarDatabase::getSampleDB();
        let c1 = Car::new(String::from("carD"), String::from("branchF"), String::from("modelA"));
        assert_eq!(uc::uc_createCar(&mut db, c1, &db_branch, &db_model).is_err(),true);
    }

    #[test]
    fn test_uc_createCar_success() {
        //CASE: CREATED SUCCESSFULLY
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let db_model = mockdb::ModelDatabase::getSampleDB();
        let mut db = mockdb::CarDatabase::getSampleDB();
        let c1 = Car::new(String::from("carD"), String::from("branchA"), String::from("modelA"));
        assert_eq!(uc::uc_createCar(&mut db, c1, &db_branch, &db_model).is_err(),false);
    }

    #[test]
    fn test_uc_queryCar_branch_rentalgroup_fail_1() {
        //BRANCH ID DOESNT EXIST
       let db = mockdb::CarDatabase::getSampleDB();
       let  db_branch = mockdb::BranchDatabase::getSampleDB();
       let  db_rentalgroup = mockdb::RentalGroupDatabase::getSampleDB();
       let  db_model = mockdb::ModelDatabase::getSampleDB();
       let  branch_id = String::from("branchNO");
       let rentalgroup_id = String::from("A");
       assert_eq!(uc::uc_queryCar_branch_rg(
        &db,
        &db_branch,
        &db_rentalgroup,
        &db_model,
        branch_id,
        rentalgroup_id).is_err(), true);
    }
    #[test]
    fn test_uc_queryCar_branch_rentalgroup_fail_2() {
        //RENTAL GROUP DOESNT EXIST
        let db = mockdb::CarDatabase::getSampleDB();
        let  db_branch = mockdb::BranchDatabase::getSampleDB();
        let  db_rentalgroup = mockdb::RentalGroupDatabase::getSampleDB();
        let  db_model = mockdb::ModelDatabase::getSampleDB();
        let  branch_id = String::from("branchA");
        let rentalgroup_id = String::from("E");
        assert_eq!(uc::uc_queryCar_branch_rg(
            &db,
            &db_branch,
            &db_rentalgroup,
            &db_model,
         branch_id,
         rentalgroup_id).is_err(), true);
    }
    #[test]
    fn test_uc_queryCar_branch_rentalgroup_succ_1() {
        //CASE: NO CARS
        let db = mockdb::CarDatabase::getSampleDB();
        let  db_branch = mockdb::BranchDatabase::getSampleDB();
        let  db_rentalgroup = mockdb::RentalGroupDatabase::getSampleDB();
        let  db_model = mockdb::ModelDatabase::getSampleDB();
        let  branch_id = String::from("branchB");
        let rentalgroup_id = String::from("B");
        assert_eq!(uc::uc_queryCar_branch_rg(
            &db,
            &db_branch,
            &db_rentalgroup,
            &db_model,
         branch_id,
         rentalgroup_id).unwrap().len(), 0);
    }
    #[test]
    fn test_uc_queryCar_branch_rentalgroup_succ_2() {
        //CASE: YES CARS
        let db = mockdb::CarDatabase::getSampleDB();
        let  db_branch = mockdb::BranchDatabase::getSampleDB();
        let  db_rentalgroup = mockdb::RentalGroupDatabase::getSampleDB();
        let  db_model = mockdb::ModelDatabase::getSampleDB();
        let  branch_id = String::from("branchA");
        let rentalgroup_id = String::from("A");
        assert_ne!(uc::uc_queryCar_branch_rg(
         &db,
         &db_branch,
         &db_rentalgroup,
         &db_model,
         branch_id,
         rentalgroup_id).unwrap().len(), 0);
    }


    //Add neighboring branches
    #[test]
    fn test_working_addneighbor() {
        //CASE: YES
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let mut db_neighbor = mockdb::BranchNeighborDatabase::getSampleDB();
        assert_eq!(uc::uc_addBranchNeighbor(String::from("branchA"), String::from("branchB"), &db_branch, &mut db_neighbor).is_ok()
        , true);
    }

    #[test]
    fn test_working_addneighbor_2() {
        //CASE: Branch not exist yet
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let mut db_neighbor = mockdb::BranchNeighborDatabase::getSampleDB();
        assert_eq!(uc::uc_addBranchNeighbor(String::from("branchC"), String::from("branchB"), &db_branch, &mut db_neighbor).is_ok()
        , false);
    }

     #[test]
    fn test_working_addneighbor_3() {
        //CASE: neighboring already exists
        let db_branch = mockdb::BranchDatabase::getSampleDB();
        let mut db_neighbor = mockdb::BranchNeighborDatabase::getSampleDB();
        uc::uc_addBranchNeighbor(String::from("branchA"), String::from("branchB"), &db_branch, &mut db_neighbor);        
        assert_eq!(uc::uc_addBranchNeighbor(String::from("branchA"), String::from("branchB"), &db_branch, &mut db_neighbor).is_ok()
        , false);
    } 

}