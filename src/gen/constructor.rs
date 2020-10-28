//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use crate::utils::sanitize;
use serde_json::json;
use crate::ffi_expose;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}});
";

static BODY_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}})
{
    new (this) {{class}}({{arguments}});
}
";

//------------------------------------------------------------------------------
pub fn handle(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    if let Some(_) = ffi_expose::get_arguments(state, entity)? {
        let parent_name = sanitize(&parent.get_display_name().unwrap());
        let name = sanitize(&entity.get_display_name().unwrap());

        // Header
        state.write_header(
            HEADER_TEMPLATE,
            &json!({"class" : parent_name,
                    "arguments": "",
            }),
        );

        // Body
        state.write_source(
            BODY_TEMPLATE,
            &json!({"class" : parent_name,
                    "arguments": "",
            }),
        );
    }

    Ok(())
}
