
pub mod car {
    use crate::entity::entity::Queryable as Queryable;
    use crate::model::model::Model as Model;
    use crate::branch::branch::Branch as Branch;

    #[derive(Clone)]
    pub enum RentalStatus {
        RENTREADY,
        REMOVED,
        HELD,
        PICKEDUP,
        RETURNED,
        EXCEPTIONAL
    }

    enum MainternanceStatus {
        SERVICENEEDED,
    }
    
    #[derive(Clone)]
    pub struct Car {
        registrationnumber : String,
        name : String,
        yearofproduction : String,
        branch : String,
        color : String,
        status : RentalStatus,
        model : String,
    }

    
    impl Car {
        pub fn new(        
            registrationnumber : String,
            branch : String,
            model : String,) -> Car {
                let status = RentalStatus::RENTREADY;
                Car {
                    registrationnumber,
                    name : String::from("car-name"),
                    yearofproduction : String::from("2004"),
                    branch,
                    color : String::from("turquoise"),
                    status : RentalStatus::RENTREADY,
                    model,
                }
        }

        pub fn getBranch(&self) -> &String {
            &self.branch
        }

        pub fn getRentalStatus(&self) -> &RentalStatus {
            &self.status
        }

        pub fn getModel(&self) -> &String {
            &self.model
        }

        pub fn returnCar(&mut self) {
            self.status = RentalStatus::RETURNED;
        }
    }

    impl Queryable for Car {
        fn getqueryfield(&self) -> &String {
            &self.registrationnumber
        }
    }



}