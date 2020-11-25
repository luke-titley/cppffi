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
void {{class}}_{{outer_method}}({{class}} * self{{comma}}{{params}})
{ {{{types}}}
    using Self = {{{cpp_class}}};
    new (self) Self({{{args}}});
}
";

//------------------------------------------------------------------------------
pub fn handle(
    info: &class_info::ClassInfo,
    state: &mut State,
    entity: clang::Entity,
) -> Result<()> {
    if let Some(args) = ffi_expose::get_arguments(state, entity)? {
        let cpp_parent_name = &info.cpp_name;
        let parent_name = info.c_name.clone();

        println!("Constructor cpp name is {}", cpp_parent_name);
        println!("Constructor parent name is {}", parent_name);

        let outer_method_name = if args.name.is_none() {
            "new".to_string()
        } else {
            args.name.unwrap().clone()
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
