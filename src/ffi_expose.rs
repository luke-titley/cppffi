//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;
use super::state::State;

//------------------------------------------------------------------------------
pub struct Arguments {
    pub arguments: std::string::String,
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
                    result = Some(Arguments {
                        arguments: child.get_display_name().unwrap(),
                    });
                    return clang::EntityVisitResult::Break;
                }

                // Ignore everything else
                _ => (),
            };
            clang::EntityVisitResult::Continue
        });
    }

    Ok(result)
}
