use parsley::{get_text_from_step, Step, StepKind};
use txml::{Template, TxmlBuilder};

/*
    This is meant to "pretty" an html document output by `html`.
    it is not used by a "doc" builder
*/

pub struct PretyHtmlBuilder {
    tags: Vec<String>,
    tab_count: usize,
    results: String,
}

impl PretyHtmlBuilder {
    pub fn new() -> PretyHtmlBuilder {
        PretyHtmlBuilder {
            tags: Vec::new(),
            tab_count: 0,
            results: "".to_string(),
        }
    }

    pub fn build(&mut self) -> String {
        self.results.clone()
    }
}

// pre elements must respsect boundaries
// that's really it use

impl TxmlBuilder for PretyHtmlBuilder {
    fn push_step(&mut self, template_str: &str, step: Step) {
        match step.kind {
            // steps
            StepKind::Tag => {
                push_element(self, get_text_from_step(template_str, &step));
            }
            StepKind::ElementClosed => {
                close_element(self, get_text_from_step(template_str, &step));
            }
            StepKind::VoidElementClosed => {
                close_void_element(self, get_text_from_step(template_str, &step));
            }
            StepKind::Attr => {
                add_attr(self, get_text_from_step(template_str, &step));
            }
            StepKind::AttrValue => {
                add_attr_value(self, get_text_from_step(template_str, &step));
            }
            StepKind::AttrValueUnquoted => {
                add_attr_value_unquoted(self, get_text_from_step(template_str, &step));
            }
            StepKind::Text => {
                push_text(self, get_text_from_step(template_str, &step));
            }
            StepKind::TailTag => {
                pop_element(self, get_text_from_step(template_str, &step));
            }
            // injections
            StepKind::AttrMapInjection => {
                push_attr_map_injection(self, get_text_from_step(template_str, &step));
            }
            StepKind::DescendantInjection => {
                push_descendant_injection(self, get_text_from_step(template_str, &step));
            }
            StepKind::InjectionSpace => {
                push_injection_space(self, get_text_from_step(template_str, &step));
            }
            StepKind::InjectionConfirmed => {
                push_injection_confirmed(self, get_text_from_step(template_str, &step));
            }
            // all other steps silently pass through
            _ => {}
        }
    }
}

fn push_element(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.tags.push(tag.to_string());

    builder.results.push('\n');
    builder.results.push_str(&"\t".repeat(builder.tab_count));
    builder.results.push_str("<");
    builder.results.push_str(tag);
}

fn close_element(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.results.push_str(">");

    builder.tab_count += 1;
}

fn close_void_element(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.tags.pop();

    builder.results.push_str(">");
}

fn pop_element(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.tags.pop();
    builder.tab_count -= 1;

    builder.results.push('\n');
    builder.results.push_str(&"\t".repeat(builder.tab_count));
    builder.results.push_str("</");
    builder.results.push_str(tag);
    builder.results.push_str(">");
}

fn push_text(builder: &mut PretyHtmlBuilder, text: &str) {
    builder.results.push_str(text);
}

fn add_attr(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.results.push(' ');
    builder.results.push_str(tag);
}

fn add_attr_value(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.results.push_str("=\"");
    builder.results.push_str(tag);
    builder.results.push('"');
}

fn add_attr_value_unquoted(builder: &mut PretyHtmlBuilder, tag: &str) {
    builder.results.push('=');
    builder.results.push_str(tag);
}

// injections
fn push_attr_map_injection(builder: &mut PretyHtmlBuilder, glyph: &str) {
    builder.results.push_str(glyph);
}

fn push_descendant_injection(builder: &mut PretyHtmlBuilder, glyph: &str) {
    builder.results.push_str(glyph);
}

fn push_injection_space(builder: &mut PretyHtmlBuilder, text: &str) {
    builder.results.push_str(text);
}

fn push_injection_confirmed(builder: &mut PretyHtmlBuilder, glyph: &str) {
    builder.results.push_str(glyph);
}
