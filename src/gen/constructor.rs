//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;

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
    _: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    // Header
    state.write_header(
        HEADER_TEMPLATE,
        &json!({"class" : parent.get_display_name().unwrap(),
                "arguments": "",
        }),
    );

    // Body
    state.write_source(
        BODY_TEMPLATE,
        &json!({"class" : parent.get_display_name().unwrap(),
                "arguments": "",
        }),
    );

    Ok(())
}
