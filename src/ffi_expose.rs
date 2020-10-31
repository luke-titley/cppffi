//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;
use super::state::State;

//------------------------------------------------------------------------------
pub struct Arguments {
    pub arguments: std::vec::Vec<std::string::String>,
}

//------------------------------------------------------------------------------
pub fn get_arguments(
    _: &mut State,
    entity: clang::Entity,
) -> Result<Option<Arguments>> {
    let mut result: Option<Arguments> = None;

    if entity.has_attributes() {
        entity.visit_children(|child, _| {
            match child.get_kind() {
                clang::EntityKind::AnnotateAttr => {
                    let name = child.get_display_name().unwrap();

                    if name.starts_with("ffi_expose") {
                        let arguments: std::vec::Vec<std::string::String> =
                            name[10..]
                                .split(" ")
                                .map(|i| i.to_string())
                                .collect();

                        result = Some(Arguments { arguments });
                        return clang::EntityVisitResult::Break;
                    }
                }

                // Ignore everything else
                _ => (),
            };
            clang::EntityVisitResult::Continue
        });
    }

    Ok(result)
}
