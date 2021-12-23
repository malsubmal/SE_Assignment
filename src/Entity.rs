pub mod entity {
    pub enum Errors {
        POSTCONDITIONFAIL,
        ALREADYEXIST,
        DBERROR,
    }

    pub trait Queryable {

        fn getqueryfield(&self) -> &String;
        fn queryobject(&self) -> Result<(), String> {Ok(())} 
        
        fn shouldexist(&self)  -> Result<(), String> {
            //post_condition defaults to true
/*             match self.entityNotExisted(){
                Ok(false) => Ok(()),
                Ok(true) => Err(String::from("not existed")),
                Err(e) => Err(e),
            } */
            Ok(())
        }

        fn entityNotExisted(&self) -> Result<bool, String> {
            match self.getqueryfield().as_str() {
                "existing" => Ok(false), 
                "problem" => Err(String::from("problem querying database")),
                _ => Ok(true),
            }
        }
    
        fn shouldnotexist(&self) ->  Result<(), String> {
                match self.entityNotExisted() {
                    Ok(true) => Ok(()),
                    Ok(false) => Err(String::from("already exists")),
                    Err(e) => Err(e),
                }
            }
    }

    pub trait Creatable {
 
    fn pushobject(&self) -> Result<(), String> {Ok(())}

    
    fn addEntity(&self) -> Result<(), String> {
            self.pushobject()
    }
     
    
    }

    }

  /*   pub fn uc_addEntity(entity : &dyn Creatable) -> Result<(), String> {
        let pre_condition = pre_addEntity(entity);
        let action = match pre_condition {
            Ok(()) => addEntity(entity),
            Err(e) => Err(e),
        };
        let post_condition = match action {
            Ok(()) => post_addEntity(entity),
            Err(e) => Err(e),
        };
        post_condition
    } */





