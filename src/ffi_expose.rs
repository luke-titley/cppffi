//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
use super::result::Result;
use super::state::State;

//------------------------------------------------------------------------------
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct FFI {
    pub name : Option<std::string::String>,
}

//------------------------------------------------------------------------------
pub fn get_arguments(
    _: &mut State,
    entity: clang::Entity,
) -> Result<Option<FFI>> {
    let mut result: Option<FFI> = None;

    if entity.has_attributes() {
        entity.visit_children(|child, _| {
            match child.get_kind() {
                clang::EntityKind::AnnotateAttr => {
                    let name = child.get_display_name().unwrap();

                    println!("name: {}", name);

                    if let Ok(arguments) = ron::de::from_str::<FFI>(&name) {
                        result = Some(arguments.clone());
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
