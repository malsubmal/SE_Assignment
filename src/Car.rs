
pub mod car {
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;
    use crate::model::model::Model as Model;
    use crate::branch::branch::Branch as Branch;

    enum CarStatus {
        RENTREADY,
        SERVICENEEDED,
        REMOVED,
        HELD,
        PICKEDUP,
        RETURNED,
        EXCEPTIONAL
    }
    
    pub struct Car {
        registrationnumber : String,
        name : String,
        yearofproduction : String,
        branch : String,
        color : String,
        status : CarStatus,
        model : String,
    }

    
    impl Car {
        pub fn new(        
            registrationnumber : String,
            branch : String,
            model : String,) -> Car {
                let status = CarStatus::RENTREADY;
                Car {
                    registrationnumber,
                    name : String::from("car-name"),
                    yearofproduction : String::from("2004"),
                    branch,
                    color : String::from("turquoise"),
                    status : CarStatus::RENTREADY,
                    model,
                }
        }

        pub fn getBranch(&self) -> &String {
            &self.branch
        }

        pub fn getModel(&self) -> &String {
            &self.model
        }
    }

    impl Queryable for Car {
        fn getqueryfield(&self) -> &String {
            &self.registrationnumber
        }
    }

    impl Creatable for Car {}



}