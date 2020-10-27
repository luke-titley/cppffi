//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;
use std::io::Write;

use crate::c_expose;
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
    if let Some(expose_arguments) =
        c_expose::get_arguments(state, entity).unwrap()
    {
        let mut result = std::vec::Vec::new();
        utils::decompose_type(&mut result, &entity.get_result_type().unwrap());

        println!("{:?}", result);

        if let Some(result_type) =
            state.supported_types.get(result.last().unwrap())
        {
            result.pop();
            result.push(result_type.to_string());
            let result_combined = result.join(" ");
            // Write out the header information
            writeln!(
                &state.out_header,
                "{}",
                state
                    .renderer
                    .render_template(
                        HEADER_TEMPLATE,
                        &json!({"return" : result_combined,
                                "name" : entity.get_display_name().unwrap(),
                                "class" : parent.get_display_name().unwrap(),
                                "arguments": "",
                        })
                    )
                    .unwrap()
            );

            // Write out the body information
            writeln!(
                &state.out_source,
                "{}",
                state
                    .renderer
                    .render_template(
                        BODY_TEMPLATE,
                        &json!({"return" : result_combined,
                                "name" : entity.get_display_name().unwrap(),
                                "class" : parent.get_display_name().unwrap(),
                                "arguments": "",
                        })
                    )
                    .unwrap()
            );
        }
    }

    Ok(())
}
