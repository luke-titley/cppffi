//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;

use crate::class_info;
use crate::ffi_expose;
use crate::utils;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{return}} {{class}}__{{{name}}} ({{class}} * this{{arguments}});
";

static BODY_TEMPLATE: &'static str = "
{{return}} {{class}}__{{{name}}} ({{class}} * this{{arguments}})
{
    return reinterpret_cast<{{{class}}}*>(this)->{{{name}}};
}
";

//------------------------------------------------------------------------------
pub fn handle(
    info: &class_info::ClassInfo,
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    if let Some(_) = ffi_expose::get_arguments(state, entity).unwrap() {
        let mut result = std::vec::Vec::new();
        utils::decompose_type(&mut result, &entity.get_result_type().unwrap());

        let mut result = info.remap_template_parameters(&result[..]);

        if let Some(result_type) =
            state.supported_types.get(result.last().unwrap())
        {
            result.pop();
            result.push(result_type.to_string());
            let result_combined = result.join(" ");

            let class_name =
                utils::sanitize(&parent.get_display_name().unwrap());

            let method_name = entity.get_name().unwrap();

            // Header
            state.write_header(
                HEADER_TEMPLATE,
                &json!({"return" : result_combined,
                        "name" : method_name,
                        "class" : class_name,
                        "arguments": "",
                }),
            );

            // Source
            state.write_source(
                BODY_TEMPLATE,
                &json!({"return" : result_combined,
                        "name" : method_name,
                        "class" : class_name,
                        "arguments": "",
                }),
            );
        }
    }

    Ok(())
}
