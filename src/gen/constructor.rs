//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::arguments::build_arguments;
use crate::class_info;
use crate::ffi_expose;
use crate::result::Result;
use crate::state::State;
use crate::utils::sanitize;
use serde_json::json;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
void {{class}}_{{outer_method}}({{class}} * self{{comma}}{{params}});
";

static BODY_TEMPLATE: &'static str = "
static_assert(sizeof({{class}}) == sizeof({{{cpp_class}}}), \"Miss matching size for {{{cpp_class}}}\");
static_assert(alignof({{class}}) == alignof({{{cpp_class}}}), \"Miss matching alignment for {{{cpp_class}}}\");

void {{class}}_{{outer_method}}({{class}} * self{{comma}}{{params}})
{ {{{types}}}
    new (self) {{{cpp_class}}}({{{args}}});
}
";

//------------------------------------------------------------------------------
pub fn handle(
    info: &class_info::ClassInfo,
    state: &mut State,
    entity: clang::Entity,
) -> Result<()> {
    if let Some(ffi_arguments) = ffi_expose::get_arguments(state, entity)? {
        let cpp_parent_name = &info.cpp_name;
        let parent_name = info.c_name.clone();

        let outer_method_name = if ffi_arguments.arguments.is_empty()
            || ffi_arguments.arguments[0].is_empty()
        {
            "new".to_string()
        } else {
            ffi_arguments.arguments[0].clone()
        };

        // Build the parameter list
        if let Some(arguments) = entity.get_arguments() {
            //let args = arguments.iter().map(|arg| {});

            let (types, params, args, comma) = build_arguments(
                info,
                state,
                &parent_name,
                &outer_method_name,
                arguments,
            );

            // Header
            state.write_header(
                HEADER_TEMPLATE,
                &json!({"class" : parent_name,
                        "outer_method" : outer_method_name,
                        "args": args,
                        "params": params,
                        "comma": comma,
                }),
            );

            // Body
            state.write_source(
                BODY_TEMPLATE,
                &json!({
                        "class" : parent_name,
                        "cpp_class" : cpp_parent_name,
                        "outer_method" : outer_method_name,
                        "args": args,
                        "params": params,
                        "comma": comma,
                        "types" : types,
                }),
            );
        }
    }

    Ok(())
}
