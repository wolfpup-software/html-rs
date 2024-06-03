use parsley::StepKind;
use static_txml_builder::{TxmlBuilder, TxmlBuilderResults};
use txml::{Component, Template};

struct TemplateBit {
    pub inj_index: usize,
}

pub enum StackBit<'a> {
    Tmpl(&'a Component, TxmlBuilderResults, TemplateBit),
    Cmpnt(&'a Component),
    None,
}

fn get_stackable(mut builder: TxmlBuilder, component: &Component) -> (TxmlBuilder, StackBit) {
    let stack_bit = match component {
        Component::Text(text) => StackBit::Cmpnt(component),
        Component::List(list) => StackBit::Cmpnt(component),
        Component::Tmpl(tmpl) => {
            StackBit::Tmpl(component, builder.build(tmpl), TemplateBit { inj_index: 0 })
        }
        _ => StackBit::None,
    };

    (builder, stack_bit)
}

fn build_template(mut builder: TxmlBuilder, component: &Component) -> String {
    let mut templ_str = "".to_string();

    let sbit;
    (builder, sbit) = get_stackable(builder, component);

    let mut stack: Vec<StackBit> = Vec::from([sbit]);
    while let Some(mut stack_bit) = stack.pop() {
        match stack_bit {
            // akin to (&componet, &results, &mut bit)
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
                                templ_str = add_attr_list(templ_str, inj);
                            }
                            // queue descendant injections to queue
                            StepKind::DescendantInjection => {
                                // push previous
                                stack.push(stack_bit);
                                let bit;
                                (builder, bit) = get_stackable(builder, inj);
                                stack.push(bit);
                            }
                            _ => {}
                        }
                    };
                }
            }
            StackBit::Cmpnt(cmpnt) => {
                match cmpnt {
                    // break lists into smaller chuncks
                    Component::List(list) => {
                        // add chunks in reverse order
                        for cmpnt in list.iter().rev() {
                            let bit;
                            (builder, bit) = get_stackable(builder, cmpnt);
                            stack.push(bit);
                        }
                    }
                    Component::Text(text) => templ_str.push_str(text),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    templ_str
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

fn add_attr_list(mut template_str: String, component: &Component) -> String {
    match component {
        Component::Attr(attr) => add_attr(template_str, attr),
        Component::AttrVal(attr, val) => add_attr_val(template_str, attr, val),
        Component::List(attrList) => {
            for cmpnt in attrList {
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
