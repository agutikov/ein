//! **The chat template, rendered in EinAf.** The model's file carries its
//! chat template as a Jinja text (`tokenizer.chat_template`); llama.cpp's C
//! renderer, `llama_chat_apply_template`, knows a fixed list of templates and
//! no Jinja, and the binding wraps nothing else. So EinAf renders the
//! template's own text with `minijinja`, as Hugging Face's renderer and
//! llama.cpp's server render it: blocks trimmed, Python's string methods
//! (`startswith`, `split`, `rstrip`), `raise_exception`, and `tojson` as
//! Python's `json.dumps` writes, keys in the order given.
//!
//! The variables the template reads: `messages`, `tools` (absent where none
//! is declared), `add_generation_prompt` (always), and `enable_thinking`,
//! the thinking switch. The rendered text is tokenised with the special
//! tokens parsed; its digest and its tokens' are the envelope's.

use minijinja::value::ValueKind;
use minijinja::{Environment, Error, ErrorKind, Value};

use super::{Failure, Message, ToolDecl};

/// What renders the template, as the envelope names it.
pub const RENDERER: &str = "minijinja 2, trim_blocks and lstrip_blocks, pycompat";

/// **Render `template` over a chat**, with the generation prompt opened and
/// the thinking switched on or off.
pub fn render(
    template: &str,
    messages: &[Message],
    tools: &[ToolDecl],
    thinking: bool,
) -> Result<String, Failure> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(false);
    env.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
    env.add_function("raise_exception", raise_exception);
    env.add_filter("tojson", tojson);
    let failure = |e: Error| Failure::Template {
        message: format!("{e:#}"),
    };
    let tmpl = env.template_from_str(template).map_err(failure)?;
    let mut ctx: Vec<(&str, Value)> = vec![
        ("messages", Value::from_serialize(messages)),
        ("add_generation_prompt", Value::from(true)),
        ("enable_thinking", Value::from(thinking)),
    ];
    if !tools.is_empty() {
        ctx.push((
            "tools",
            Value::from(tools.iter().map(tool).collect::<Vec<_>>()),
        ));
    }
    let ctx: Value = ctx.into_iter().collect();
    tmpl.render(ctx).map_err(failure)
}

/// A declared function as the chat API writes it, keys in this order.
fn tool(t: &ToolDecl) -> Value {
    let properties: Value = t
        .parameters
        .iter()
        .map(|p| {
            let schema: Value = [
                ("type", Value::from("string")),
                ("description", Value::from(p.description.as_str())),
            ]
            .into_iter()
            .collect();
            (p.name.as_str(), schema)
        })
        .collect();
    let required: Vec<Value> = t
        .parameters
        .iter()
        .map(|p| Value::from(p.name.as_str()))
        .collect();
    let parameters: Value = [
        ("type", Value::from("object")),
        ("properties", properties),
        ("required", Value::from(required)),
    ]
    .into_iter()
    .collect();
    let function: Value = [
        ("name", Value::from(t.name.as_str())),
        ("description", Value::from(t.description.as_str())),
        ("parameters", parameters),
    ]
    .into_iter()
    .collect();
    [("type", Value::from("function")), ("function", function)]
        .into_iter()
        .collect()
}

fn raise_exception(message: String) -> Result<Value, Error> {
    Err(Error::new(ErrorKind::InvalidOperation, message))
}

/// `tojson` as Python's `json.dumps` writes a value: `", "` and `": "`
/// between items, keys in the order given, every character as it is.
fn tojson(value: Value) -> Result<Value, Error> {
    let mut out = String::new();
    dump(&value, &mut out)?;
    Ok(Value::from_safe_string(out))
}

fn dump(value: &Value, out: &mut String) -> Result<(), Error> {
    match value.kind() {
        ValueKind::Undefined | ValueKind::None => out.push_str("null"),
        ValueKind::Bool => out.push_str(if value.is_true() { "true" } else { "false" }),
        ValueKind::Number => out.push_str(&value.to_string()),
        ValueKind::String => {
            let s = value.as_str().unwrap_or_default();
            out.push_str(&serde_json::to_string(s).expect("a string is JSON"));
        }
        ValueKind::Seq | ValueKind::Iterable => {
            out.push('[');
            for (i, item) in value.try_iter()?.enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                dump(&item, out)?;
            }
            out.push(']');
        }
        ValueKind::Map => {
            out.push('{');
            for (i, key) in value.try_iter()?.enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                dump(&key, out)?;
                out.push_str(": ");
                dump(&value.get_item(&key)?, out)?;
            }
            out.push('}');
        }
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidOperation,
                format!("tojson: no JSON for a value of kind {:?}", value.kind()),
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const QWEN: &str = include_str!("../../tests/fixtures/model/chat_template.jinja");

    #[test]
    fn the_thinking_switch_opens_or_closes_the_thinking_block() {
        let m = [Message::new("user", "Who is the youngest?")];
        let on = render(QWEN, &m, &[], true).unwrap();
        let off = render(QWEN, &m, &[], false).unwrap();
        assert_eq!(
            on,
            "<|im_start|>user\nWho is the youngest?<|im_end|>\n<|im_start|>assistant\n<think>\n"
        );
        assert_eq!(
            off,
            "<|im_start|>user\nWho is the youngest?<|im_end|>\n<|im_start|>assistant\n<think>\n\n</think>\n\n"
        );
    }

    #[test]
    fn a_system_message_and_a_declared_function_are_rendered() {
        let m = [
            Message::new("system", "You write Ein."),
            Message::new("user", "Run it."),
        ];
        let t = [ToolDecl {
            name: "ein_solve".to_string(),
            description: "Run an Ein program".to_string(),
            parameters: vec![super::super::ToolParam {
                name: "program".to_string(),
                description: "the program's text".to_string(),
            }],
        }];
        let r = render(QWEN, &m, &t, false).unwrap();
        assert!(r.starts_with("<|im_start|>system\n# Tools\n\nYou have access"));
        assert!(r.contains(
            "{\"type\": \"function\", \"function\": {\"name\": \"ein_solve\", \"description\": \"Run an Ein program\", \"parameters\": {\"type\": \"object\", \"properties\": {\"program\": {\"type\": \"string\", \"description\": \"the program's text\"}}, \"required\": [\"program\"]}}}"
        ));
        assert!(r.contains("</IMPORTANT>\n\nYou write Ein.<|im_end|>\n<|im_start|>user\nRun it."));
    }

    #[test]
    fn a_chat_with_no_question_is_a_failure_of_the_template() {
        let e = render(QWEN, &[], &[], false).unwrap_err();
        assert!(
            matches!(&e, Failure::Template { message } if message.contains("No messages provided")),
            "{e:?}"
        );
    }
}
