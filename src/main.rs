//------------------------------------------------------------------------------
// Copywrite Luke Titley 2020
//------------------------------------------------------------------------------
type Result<R> = std::result::Result<R, std::string::String>;

use handlebars;
use serde_json::json;
use std::collections::HashMap;

use std::io::Write;

static CLASS_TEMPLATE: &'static str = "
typedef struct
{
    char m_impl[{{size}}];
} __attribute__((aligned({{align}}))) {{name}};
";

static CONSTRUCTOR_HEADER_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}});
";

static CONSTRUCTOR_BODY_TEMPLATE: &'static str = "
{{class}}_{{class}} ({{class}} * this{{arguments}})
{
    new (this) {{class}}({{arguments}});
}
";

static METHOD_HEADER_TEMPLATE: &'static str = "
{{return}} {{class}}__{{name}} ({{class}} * this{{arguments}});
";

static METHOD_BODY_TEMPLATE: &'static str = "
{{return}} {{class}}__{{name}} ({{class}} * this{{arguments}})
{
    return reinterpret_cast<{{class}}*>(this)->{{name}};
}
";

//------------------------------------------------------------------------------
type SupportedTypes = HashMap<std::string::String, std::string::String>;

//------------------------------------------------------------------------------
struct State<'a> {
    renderer: handlebars::Handlebars<'a>,
    buffer: std::string::String,
    supported_types: SupportedTypes,
    out_source: std::fs::File,
    out_header: std::fs::File,
}

//------------------------------------------------------------------------------
fn handle_constructor(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    // Header
    writeln!(
        &state.out_header,
        "{}",
        state
            .renderer
            .render_template(
                CONSTRUCTOR_HEADER_TEMPLATE,
                &json!({"class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                })
            )
            .unwrap()
    );

    // Body
    writeln!(
        &state.out_source,
        "{}",
        state
            .renderer
            .render_template(
                CONSTRUCTOR_BODY_TEMPLATE,
                &json!({"class" : parent.get_display_name().unwrap(),
                        "arguments": "",
                })
            )
            .unwrap()
    );

    Ok(())
}

//------------------------------------------------------------------------------
fn decompose_type<'a, 'b>(
    result: &'b mut std::vec::Vec<std::string::String>,
    type_: &'a clang::Type<'a>,
) {
    if let Some(type_) = type_.get_pointee_type() {
        result.push("*".to_string());
        decompose_type(result, &type_)
    }
    else if type_.is_const_qualified() {
        result.push("const".to_string());
        result.push(type_.get_display_name()[6..].to_string());
    }
    else {
        result.push(type_.get_display_name())
    }
}

//------------------------------------------------------------------------------
fn handle_method(
    state: &mut State,
    entity: clang::Entity,
    parent: clang::Entity,
) -> Result<()> {
    let mut result = std::vec::Vec::new();
    decompose_type(&mut result, &entity.get_result_type().unwrap());

    println!("{:?}", result);

    if let Some(result) = state.supported_types.get(result.last().unwrap()) {
        // Write out the header information
        writeln!(
            &state.out_header,
            "{}",
            state
                .renderer
                .render_template(
                    METHOD_HEADER_TEMPLATE,
                    &json!({"return" : result,
                            "name" : entity.get_display_name().unwrap(),
                            "class" : parent.get_display_name().unwrap(),
                            "arguments": "",
                    })
                )
                .unwrap()
        );

        // Write out the body information
        writeln!(
            &state.out_source,
            "{}",
            state
                .renderer
                .render_template(
                    METHOD_BODY_TEMPLATE,
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
    let name = entity.get_display_name().unwrap();

    // Generate the code for the class
    writeln!(
        &state.out_header,
        "{}",
        state
            .renderer
            .render_template(
                CLASS_TEMPLATE,
                &json!({"name" : name,
                    "size" : size,
                    "align" : align})
            )
            .unwrap()
    );

    // Add the class to the list of supported types subsequent methods
    // will be able to refer to it
    state
        .supported_types
        .insert(name.to_string(), name.to_string());

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
        out_source: std::fs::File::create("out.cpp").unwrap(),
        out_header: std::fs::File::create("out.hpp").unwrap(),
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
