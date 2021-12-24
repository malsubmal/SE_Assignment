pub mod usecase {
    pub struct Usecase<'a>
    {
        precondition : &'a Fn() -> Result<(), String>,
        action :  &'a mut FnMut() -> Result<(), String>,
        postcondition :  &'a Fn() -> Result<(), String>,
    }

    impl<'a> Usecase<'a> 
    {
        pub fn new(
        precondition : &'a Fn() -> Result<(), String>,
        action :  &'a mut FnMut() -> Result<(), String>,
        postcondition :  &'a Fn() -> Result<(), String>
        ) -> Usecase<'a> {
            Usecase{
                precondition,
                action,
                postcondition
            }
        }

        pub fn run(&'a mut self) -> Result<(), String> {
            let action = match (self.precondition)() {
                Ok(()) => (self.action)(),
                Err(e) => Err(e),
            };
            let post_condition = match action {
                Ok(()) => (self.postcondition)(),
                Err(e) => Err(e),
            };
            post_condition
        }
    }
}

