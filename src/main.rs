//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
type Result<R> = std::result::Result<R, std::string::String>;

use handlebars;
use serde_json::json;
use std::collections::HashMap;

static CLASS_TEMPLATE: &'static str = "
typedef struct
{
    char m_impl[{{size}}];
} __attribute__((aligned({{align}}))) {{name}};
";

static CONSTRUCTOR_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}});
";

static METHOD_TEMPLATE: &'static str = "
{{return}} {{class}}_{{name}} ({{class}} * this{{arguments}});
";

//------------------------------------------------------------------------------
type SupportedTypes = HashMap<std::string::String, std::string::String>;

//------------------------------------------------------------------------------
struct State<'a> {
    renderer: handlebars::Handlebars<'a>,
    buffer: std::string::String,
    supported_types: SupportedTypes,
}

//------------------------------------------------------------------------------
fn handle_constructor(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    println!(
        "{}",
        state
            .renderer
            .render_template(
                CONSTRUCTOR_TEMPLATE,
                &json!({"class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                })
            )
            .unwrap()
    );

    Ok(())
}

//------------------------------------------------------------------------------
fn handle_method(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    //let result = entity.get_result_type().unwrap().get_canonical_type();
    let result = entity.get_result_type().unwrap();

    //print!("{}", result.get_display_name());

    if let Some(result) = state
        .supported_types
        .get(&result.get_display_name().to_string())
    {
        println!(
            "{}",
            state
                .renderer
                .render_template(
                    METHOD_TEMPLATE,
                    &json!({"return" : result,
                            "name" : entity.get_display_name().unwrap(),
                            "class" : parent.get_display_name().unwrap(),
                            "arguments": "",
                    })
                )
                .unwrap()
        );
    }

    Ok(())
}

//------------------------------------------------------------------------------
fn handle_class(state: &mut State, entity: clang::Entity) -> Result<()> {
    let size = entity.get_type().unwrap().get_sizeof().unwrap();
    let align = entity.get_type().unwrap().get_alignof().unwrap();

    // Generate the code for the class
    println!(
        "{}",
        state
            .renderer
            .render_template(
                CLASS_TEMPLATE,
                &json!({"name" : entity.get_display_name().unwrap(),
                    "size" : size,
                    "align" : align})
            )
            .unwrap()
    );

    // Generate the methods of the class
    entity.visit_children(|child, parent| {
        match child.get_kind() {
            // Constructor
            clang::EntityKind::Constructor => {
                handle_constructor(state, child, entity);
            }

            // Methods
            clang::EntityKind::Method => {
                handle_method(state, child, entity);
            }

            // Ignore everything else
            _ => (),
        };
        clang::EntityVisitResult::Continue
    });

    Ok(())
}

//------------------------------------------------------------------------------
fn register_supported_types(
    supported_types: &mut SupportedTypes,
) -> Result<()> {
    for i in ["int", "float", "double", "char"].iter() {
        supported_types.insert(i.to_string(), i.to_string());
    }

    Ok(())
}

//------------------------------------------------------------------------------
fn doit() -> Result<()> {
    // The state we'll pass around during codegen
    let mut state = State {
        renderer: handlebars::Handlebars::new(),
        buffer: std::string::String::new(),
        supported_types: SupportedTypes::new(),
    };
    register_supported_types(&mut state.supported_types);

    // Start parsing header files
    let clang = clang::Clang::new()?;
    let index = clang::Index::new(&clang, true, true);

    let parser = index.parser("example_class.hpp");
    let translation_unit = parser.parse().unwrap();

    // Loop over the header file exporting everything that is of interest
    translation_unit
        .get_entity()
        .visit_children(|entity, parent| {
            match entity.get_kind() {
                // We want to output classes
                clang::EntityKind::ClassDecl => {
                    handle_class(&mut state, entity);
                }

                // Ignore everything else
                _ => (),
            };
            clang::EntityVisitResult::Continue
        });

    Ok(())
}

//------------------------------------------------------------------------------
fn main() {
    doit();
}
