//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::constructor;
use super::field;
use super::method;
use crate::class_info;
use crate::ffi_expose;
use crate::result::Result;
use crate::state::State;
use crate::utils::{sanitize, to_visit_result};
use serde_json::json;

//------------------------------------------------------------------------------
static HEADER_TEMPLATE: &'static str = "
typedef struct { FFI_SIZE({{size}}) } FFI_ALIGN({{align}}) {{name}};
";

//------------------------------------------------------------------------------
pub fn handle(
    ns: &str,
    state: &mut State,
    entity: clang::Entity,
) -> Result<()> {
    if let Some(ffi_arguments) = ffi_expose::get_arguments(state, entity)? {
        let cpp_name = entity.get_display_name().unwrap();

        let name = &format!(
            "{}__{}",
            ns,
            sanitize(if ffi_arguments.arguments.is_empty() {
                &cpp_name
            } else {
                &ffi_arguments.arguments[0]
            })
        );

        let original_name = format!("{}::{}", ns, cpp_name);

        let size = entity.get_type().unwrap().get_sizeof().unwrap();
        let align = entity.get_type().unwrap().get_alignof().unwrap();

        // Generate the code for the class
        state.write_header(
            HEADER_TEMPLATE,
            &json!({"name" : name,
                    "size" : size,
                    "align" : align}),
        );

        // Add the class to the list of supported types subsequent methods
        // will be able to refer to it
        state.supported_types.insert(
            entity.get_display_name().unwrap().to_string(),
            name.to_string(),
        );

        if let Some(definition) = entity.get_template() {
            if let Some(_) = entity.get_template_arguments() {
                //println!("We have template arguments {:?}", arguments);
            }

            // Falling back to regex here. Not brilliant.
            let info = class_info::ClassInfo {
                template_parameters:
                    class_info::build_template_parameter_mapping(
                        definition, entity,
                    ),
                c_name: name.clone(),
                namespace: ns.to_string(),
                cpp_name: original_name.clone(),
            };

            //println!("{:?}", &info.template_parameters);

            /*
            println!("{}", entity.get_display_name().unwrap());
            println!("{}", definition.get_display_name().unwrap());
            */

            // Generate the methods of the class template definition
            definition.visit_children(|child, _| {
                match child.get_kind() {
                    // Constructor
                    clang::EntityKind::Constructor => {
                        to_visit_result(constructor::handle(
                            &info, state, child,
                        ));
                    }

                    // Fields
                    clang::EntityKind::FieldDecl => {
                        to_visit_result(field::handle(
                            &info, state, child, entity,
                        ));
                    }

                    // Methods
                    clang::EntityKind::Method => {
                        to_visit_result(method::handle(&info, state, child));
                    }

                    // Ignore everything else
                    _ => (),
                };
                clang::EntityVisitResult::Continue
            });
        }

        // Generate the methods of the class
        entity.visit_children(|child, _| {
            let info = class_info::ClassInfo::new(&name, ns, &original_name);

            match child.get_kind() {
                // Constructor
                clang::EntityKind::Constructor => {
                    to_visit_result(constructor::handle(&info, state, child));
                }

                // Fields
                clang::EntityKind::FieldDecl => {
                    to_visit_result(field::handle(&info, state, child, entity));
                }

                // Methods
                clang::EntityKind::Method => {
                    to_visit_result(method::handle(&info, state, child));
                }

                // Ignore everything else
                _ => (),
            };
            clang::EntityVisitResult::Continue
        });
    }

    Ok(())
}
