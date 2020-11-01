//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use crate::result::Result;
use crate::state::State;
use serde_json::json;

use crate::arguments::{build_arguments, convert_to_c_type};
use crate::class_info;
use crate::ffi_expose;
use crate::utils;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
{{return}} {{class}}__{{{outer_name}}}({{class}} * this{{comma}}{{params}});
";

static BODY_TEMPLATE: &'static str = "
{{return}} {{class}}__{{{outer_name}}}({{class}} * this{{comma}}{{params}})
{ {{{types}}}
    {{#if is_void}}
    return ffi_cast<{{{class}}} >(this).{{{name}}}({{{args}}});
    {{else}}
    {{return}} ffi_result =
        ffi_cast<{{{cpp_class}}} >(this).{{{name}}}({{{args}}}
        )
    );
    return ffi_cast<{{return}}>(&ffi_result);
    {{/if}}
}
";

//------------------------------------------------------------------------------
pub fn handle(
    info: &class_info::ClassInfo,
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    if let Some(ffi_arguments) =
        ffi_expose::get_arguments(state, entity).unwrap()
    {
        if let Some(result_type) =
            convert_to_c_type(info, state, &entity.get_result_type().unwrap())
        {
            let cpp_class_name = parent.get_display_name().unwrap();
            let class_name = info.c_name.clone();

            let method_name = entity.get_name().unwrap();
            let outer_method_name = if ffi_arguments.arguments.is_empty()
                || ffi_arguments.arguments[0].is_empty()
            {
                method_name.clone()
            } else {
                ffi_arguments.arguments[0].clone()
            };

            // Build the parameter list
            if let Some(arguments) = entity.get_arguments() {
                //let args = arguments.iter().map(|arg| {});

                let (types, params, args, comma) = build_arguments(
                    info,
                    state,
                    &class_name,
                    &method_name,
                    arguments,
                );

                // Header
                state.write_header(
                    HEADER_TEMPLATE,
                    &json!({"return" : result_type,
                            "name" : method_name,
                            "outer_name" : outer_method_name,
                            "class" : class_name,
                            "comma" : comma,
                            "params": params,
                            "args": args,
                    }),
                );

                // Source
                let is_void = result_type == "void";
                state.write_source(
                    BODY_TEMPLATE,
                    &json!({"return" : result_type,
                            "is_void" : is_void,
                            "name" : method_name,
                            "outer_name" : outer_method_name,
                            "class" : class_name,
                            "cpp_class" : cpp_class_name,
                            "types" : types,
                            "comma" : comma,
                            "params": params,
                            "args" : args,
                    }),
                );
            }
        }
    }

    Ok(())
}
