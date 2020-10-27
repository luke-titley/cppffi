//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;
use std::io::Write;

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
    // Header
    writeln!(
        &state.out_header,
        "{}",
        state
            .renderer
            .render_template(
                HEADER_TEMPLATE,
                &json!({"class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                })
            )
            .unwrap()
    );

    // Body
    writeln!(
        &state.out_source,
        "{}",
        state
            .renderer
            .render_template(
                BODY_TEMPLATE,
                &json!({"class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                })
            )
            .unwrap()
    );

    Ok(())
}
