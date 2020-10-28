//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::ffi_expose;
use crate::result::Result;
use crate::state::State;
use crate::utils::sanitize;
use serde_json::json;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}});
";

static BODY_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}})
{
    {{body}}
}
";


//------------------------------------------------------------------------------
pub fn handle(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    if let Some(_) = ffi_expose::get_arguments(state, entity)? {
        let cpp_parent_name = parent.get_display_name().unwrap();
        let parent_name = sanitize(&cpp_parent_name);
        let name = sanitize(&entity.get_display_name().unwrap());

        // Header
        state.write_header(
            HEADER_TEMPLATE,
            &json!({"class" : parent_name,
                    "arguments": "",
            }),
        );

        // Body
        let body = format!("new (this) {class}({arguments});", class=cpp_parent_name, arguments="");
        state.write_source(
            BODY_TEMPLATE,
            &json!({
                    "class" : parent_name,
                    "body" : body,
            }),
        );
    }

    Ok(())
}
