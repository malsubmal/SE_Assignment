pub mod entity {
    use std::collections::HashMap;


    pub trait Queryable {

        fn getqueryfield(&self) -> &String;

        fn queryobject(&self) -> Result<(), String> {Ok(())} 
        
        fn shouldexist(&self)  -> Result<(), String> {
            Ok(())
        }

        fn entityNotExisted(&self) -> Result<bool, String> {
/*             match self.getqueryfield().as_str() {
                "existing" => Ok(false), 
                "problem" => Err(String::from("problem querying database")),
                _ => Ok(true),
            } */
            Ok(true)
        }
    
        fn shouldnotexist(&self) ->  Result<(), String> {
/*                 match self.entityNotExisted() {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(String::from("already exists")),
                    Err(e) => Err(e),
                } */
                Ok(())
            }
    }

    pub trait Creatable {
 
    fn pushobject(&self) -> Result<(), String> {Ok(())}

    
    fn addEntity(&self) -> Result<(), String> {
            self.pushobject()
    }
     
    
    }

    }






