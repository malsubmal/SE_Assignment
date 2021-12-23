pub mod model {
    use std::collections::HashMap;
    use crate::entity::entity::Creatable as Creatable;
    use crate::entity::entity::Queryable as Queryable;
    use crate::rentalgroup::rentalgroup::RentalGroup as RentalGroup;
        
    pub enum ModelType {
        Automatic,
        Manual
    }

    pub struct Model {
    modelnumber : String,
    description : String,
    modeltype : ModelType,
    modelgroup : RentalGroup,
    petrolconsumption : f32,
    numberofdoors : i32,
    }

    impl Model {
        pub fn new(    
            modelnumber : String,
            modeltype : ModelType,
            modelgroup : RentalGroup) -> Model {
            Model {
                modelnumber ,
                description : String::from("default description"),
                modeltype ,
                modelgroup ,
                petrolconsumption : 4.0,
                numberofdoors : 4,
                }
        }
    }

    impl Queryable for Model {
        fn getqueryfield(&self) -> &String {
            &self.modelnumber
        }
    }
    
    impl Creatable for Model {}
}