
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
        branch : Branch,
        color : String,
        status : CarStatus,
        model : Model,
    }

    
    impl Car {
        pub fn new(        
            registrationnumber : String,
            name : String,
            yearofproduction : String,
            branch : Branch,
            color : String,
            model : Model,) -> Car {
                let status = CarStatus::RENTREADY;
                Car {
                    registrationnumber,
                    name,
                    yearofproduction,
                    branch,
                    color,
                    status,
                    model,
                }
        }
    }

    impl Queryable for Car {
        fn getqueryfield(&self) -> &String {
            &self.registrationnumber
        }
    }

    impl Creatable for Car {}



}