//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::constructor;
use super::method;
use crate::arguments::convert_to_c_type;
use crate::class_info;
use crate::ffi_expose;
use crate::result::Result;
use crate::state::State;
use crate::utils::{sanitize, to_visit_result};
use serde_json::json;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{field_type}} * {{class}}_mut_{{{field_name}}}({{class}} * this);
const {{field_type}} * {{class}}_get_{{{field_name}}}({{class}} * this);
";

static BODY_TEMPLATE: &'static str = "
{{field_type}} * {{class}}_mut_{{{field_name}}}({{class}} * this)
{
    return & ffi_cast<{{field_type}}>(&
        ffi_cast<{{{cpp_class}}} >(this).{{{field_name}}}
        )
    );
}
const {{field_type}} * {{class}}_get_{{{field_name}}}(const {{class}} * this)
{
    return & ffi_cast<{{field_type}}>(&
        ffi_cast<{{{cpp_class}}} >(this).{{{field_name}}}
        )
    );
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
        if let Some(field_type) =
            convert_to_c_type(info, state, &entity.get_type().unwrap())
        {
            let cpp_class_name = parent.get_display_name().unwrap();
            let class_name = info.c_name.clone();
            let field_name = entity.get_display_name().unwrap();

            state.write_header(
                HEADER_TEMPLATE,
                &json!({"field_type" : field_type,
                        "class" : class_name,
                        "field_name" : field_name,
                }),
            );

            state.write_source(
                BODY_TEMPLATE,
                &json!({"field_type" : field_type,
                        "class" : class_name,
                        "field_name" : field_name,
                        "cpp_class" : cpp_class_name,
                }),
            );
        }
    }

    Ok(())
}
