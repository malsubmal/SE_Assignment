pub mod rentalgroup {
    use crate::entity::entity::Queryable as Queryable;
    
    pub struct RentalGroup {
    name : String,
    price : String,
    }
    
    impl RentalGroup {
        pub fn new(
            name : String,
            price : String) -> RentalGroup {
                RentalGroup {
                    name,
                    price,
                }
        }
    }
    
    impl Queryable for RentalGroup {
        fn getqueryfield(&self) -> &String {
            &self.name
        }
    }
    
}