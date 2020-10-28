//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::constructor;
use super::method;
use crate::result::Result;
use crate::state::State;
use serde_json::json;

use crate::c_expose;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
typedef struct
{
    char m_impl[{{size}}];
} __attribute__((aligned({{align}}))) {{name}};
";

//------------------------------------------------------------------------------
pub fn handle(state: &mut State, entity: clang::Entity) -> Result<()> {
    if let Some(_) = c_expose::get_arguments(state, entity).unwrap() {
        //println!("Weve found something");
        println!("{}", entity.get_display_name().unwrap());

        let underlying_type =  entity
        .get_typedef_underlying_type()
        .unwrap();

        println!("{}", underlying_type.get_display_name());
        println!("{}", underlying_type.get_sizeof().unwrap());

        /*
        println!(
            "{}",
            entity
                .get_typedef_underlying_type()
                .unwrap()
                .get_display_name()
        );
        */
    }

    //println!("{}", entity.get_display_name().unwrap());

    Ok(())
}
