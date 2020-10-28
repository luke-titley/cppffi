//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;

use crate::ffi_expose;
use crate::utils;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{return}} {{class}}__{{name}} ({{class}} * this{{arguments}});
";

static BODY_TEMPLATE: &'static str = "
{{return}} {{class}}__{{name}} ({{class}} * this{{arguments}})
{
    return reinterpret_cast<{{class}}*>(this)->{{name}};
}
";

//------------------------------------------------------------------------------
pub fn handle(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {

    if let Some(_) = ffi_expose::get_arguments(state, entity).unwrap() {
        let mut result = std::vec::Vec::new();
        utils::decompose_type(&mut result, &entity.get_result_type().unwrap());

        //println!("{:?}", result);

        println!(
            "{} {}",
            entity.get_display_name().unwrap(),
            entity.has_attributes()
        );

        if let Some(result_type) =
            state.supported_types.get(result.last().unwrap())
        {
            result.pop();
            result.push(result_type.to_string());
            let result_combined = result.join(" ");

            // Header
            state.write_header(
                HEADER_TEMPLATE,
                &json!({"return" : result_combined,
                        "name" : entity.get_display_name().unwrap(),
                        "class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                }),
            );

            // Source
            state.write_source(
                BODY_TEMPLATE,
                &json!({"return" : result_combined,
                        "name" : entity.get_display_name().unwrap(),
                        "class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                }),
            );
        }
    }

    Ok(())
}
