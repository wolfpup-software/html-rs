use coyote::Component;
use parse::StepKind;
use txml_string::{build, Results};

struct TemplateBit {
    pub inj_index: usize,
}

enum StackBit<'a> {
    Tmpl(&'a Component, Results, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

// this needs a context to retain a scope for chaching.
// this way renders are

pub struct TemplateBuilder {}

impl TemplateBuilder {
    fn new() -> TemplateBuilder {
        TemplateBuilder {}
    }

    fn build(&mut self, template_str: &str) -> Results {
        // chance to cache templates here
        build(template_str)
    }
}

trait TemplateBuilderImpl {
    fn build(&mut self, template_str: &str) -> Results;
}
// There are three sections that are basically copy paste.
// They need to be in the same function / ownership scope?
// Cannot mutable borrow more than once.
// Rail programming makes it impossible to swap a self.propert
// (builder, template) = build(builder, template_str)
// (self.builder, template) = build(builer, template_str)

pub fn build_template(builder: &mut dyn TemplateBuilderImpl, component: &Component) -> String {
    let mut templ_str = "".to_string();

    let sbit = match component {
        Component::Text(_text) => StackBit::Cmpnt(component),
        Component::List(_list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            // chance to cache templates here
            let txml_literal = builder.build(&tmpl.template_str);
            StackBit::Tmpl(component, txml_literal, TemplateBit { inj_index: 0 })
        }
        _ => StackBit::None,
    };

    let mut stack: Vec<StackBit> = Vec::from([sbit]);
    while let Some(mut stack_bit) = stack.pop() {
        match stack_bit {
            // text or list
            StackBit::Cmpnt(cmpnt) => match cmpnt {
                Component::Text(text) => templ_str.push_str(text),
                Component::List(list) => {
                    for cmpnt in list.iter().rev() {
                        let bit = match cmpnt {
                            Component::Text(_text) => StackBit::Cmpnt(cmpnt),
                            Component::List(_list) => StackBit::Cmpnt(cmpnt),
                            Component::Tmpl(tmpl) => {
                                // chance to cache templates here
                                let txml_literal = builder.build(&tmpl.template_str);
                                StackBit::Tmpl(cmpnt, txml_literal, TemplateBit { inj_index: 0 })
                            }
                            _ => StackBit::None,
                        };
                        stack.push(bit);
                    }
                    continue;
                }
                _ => {}
            },
            StackBit::Tmpl(component, ref results, ref mut bit) => {
                // templates will be N + 1
                // injections will be length N
                let index = bit.inj_index;
                bit.inj_index += 1;

                // add template
                if let Some(chunk) = results.strs.get(index) {
                    templ_str.push_str(chunk);
                }
                // add injection
                if let Component::Tmpl(template) = component {
                    // if there is an injection
                    if let (Some(inj_kind), Some(inj)) =
                        (results.injs.get(index), template.injections.get(index))
                    {
                        match inj_kind {
                            // add attribute injections to template
                            StepKind::AttrMapInjection => {
                                templ_str = add_attr_inj(templ_str, inj);
                            }
                            // queue descendant injections to queue
                            StepKind::DescendantInjection => {
                                // push template back and bail early
                                stack.push(stack_bit);

                                let bit = match inj {
                                    Component::Text(_text) => StackBit::Cmpnt(inj),
                                    Component::List(_list) => StackBit::Cmpnt(inj),
                                    Component::Tmpl(tmpl) => {
                                        // chance to cache templates here
                                        let txml_literal = builder.build(&tmpl.template_str);
                                        StackBit::Tmpl(
                                            inj,
                                            txml_literal,
                                            TemplateBit { inj_index: 0 },
                                        )
                                    }
                                    _ => StackBit::None,
                                };
                                stack.push(bit);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    // don't forget the last part of the templates!
                    if index < results.strs.len() {
                        stack.push(stack_bit);
                    }
                }
            }
            _ => {}
        }
    }

    templ_str
}

fn add_attr_inj(mut template_str: String, component: &Component) -> String {
    match component {
        Component::Attr(attr) => add_attr(template_str, attr),
        Component::AttrVal(attr, val) => add_attr_val(template_str, attr, val),
        Component::List(attr_list) => {
            for cmpnt in attr_list {
                match cmpnt {
                    Component::Attr(attr) => {
                        template_str = add_attr(template_str, &attr);
                    }
                    Component::AttrVal(attr, val) => {
                        template_str = add_attr_val(template_str, &attr, &val);
                    }
                    _ => {}
                }
            }
            template_str
        }
        _ => template_str,
    }
}

fn add_attr(mut templ_str: String, attr: &str) -> String {
    templ_str.push_str(" ");
    templ_str.push_str(attr);
    templ_str.push_str(" ");

    templ_str
}

fn add_attr_val(mut templ_str: String, attr: &str, val: &str) -> String {
    templ_str.push_str(" ");
    templ_str.push_str(attr);
    templ_str.push_str("=\"");
    templ_str.push_str(val);
    templ_str.push_str("\" ");

    templ_str
}
