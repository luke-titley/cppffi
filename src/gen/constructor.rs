//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::class_info;
use crate::ffi_expose;
use crate::result::Result;
use crate::state::State;
use crate::utils::sanitize;
use serde_json::json;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
void {{class}}_{{outer_method}} ({{class}} * this{{arguments}});
";

static BODY_TEMPLATE: &'static str = "
void {{class}}_{{outer_method}} ({{class}} * this{{arguments}})
{
    new (this) {{{cpp_class}}}({{arguments}});
}
";

//------------------------------------------------------------------------------
pub fn handle(
    info: &class_info::ClassInfo,
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    if let Some(ffi_arguments) = ffi_expose::get_arguments(state, entity)? {
        let cpp_parent_name = parent.get_display_name().unwrap();
        let parent_name = sanitize(&cpp_parent_name);

        let outer_method_name = if ffi_arguments.arguments.is_empty()
            || ffi_arguments.arguments[0].is_empty()
        {
            "new".to_string()
        } else {
            ffi_arguments.arguments[0].clone()
        };

        // Header
        state.write_header(
            HEADER_TEMPLATE,
            &json!({"class" : parent_name,
                    "outer_method" : outer_method_name,
                    "arguments": "",
            }),
        );

        // Body
        state.write_source(
            BODY_TEMPLATE,
            &json!({
                    "class" : parent_name,
                    "cpp_class" : cpp_parent_name,
                    "outer_method" : outer_method_name,
                    "arguments" : "",
            }),
        );
    }

    Ok(())
}
