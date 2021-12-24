pub mod model {
    use std::collections::HashMap;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;
        
    pub enum ModelType {
        Automatic,
        Manual
    }

    pub struct Model {
    modelnumber : String,
    description : String,
    modeltype : ModelType,
    rentalgroup : String,
    petrolconsumption : f32,
    numberofdoors : i32,
    }

    impl Model {
        pub fn new(    
            modelnumber : String,
            rentalgroup : String) -> Model {
            Model {
                modelnumber ,
                description : String::from("default description"),
                modeltype : ModelType::Automatic ,
                rentalgroup ,
                petrolconsumption : 4.0,
                numberofdoors : 4,
                }
        }

        pub fn getrentalgroup(&self) -> &String {
            &self.rentalgroup
        }
    }

    impl Queryable for Model {
        fn getqueryfield(&self) -> &String {
            &self.modelnumber
        }
    }
    
    impl Creatable for Model {}
}