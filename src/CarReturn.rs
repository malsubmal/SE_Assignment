pub mod carreturn {
    
    use crate::car::car::Car as Car;
    use crate::customer::customer::Customer as Customer;

    pub struct CarReturn {
    car : String,
    customer : String,
    daytime : String,
    }

    impl CarReturn {
        pub fn new(car : String,
            customer : String,
            daytime : String,) -> CarReturn {
                CarReturn {
                    car,
                    customer,
                    daytime,
                    }

        }
        pub fn record(&self) -> Result<(), String> {
            //fake saving operation inserted here 
            //not implemented for the prototype
            
            Ok(())
        }
    }

}