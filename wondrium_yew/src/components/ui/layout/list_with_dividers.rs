use yew::prelude::*;

use super::divider::Divider;

#[derive(PartialEq)]
pub enum ListVariant {
    VerticalList,
    TwoColList
}

#[derive(Properties, PartialEq)]
pub struct ListWithDividersProps {
    #[prop_or_default]
    pub children: Children,
    pub variant: ListVariant,
}


#[function_component(ListWithDividers)]
pub fn list_with_dividers(props: &ListWithDividersProps) -> Html {
    match props.variant {
        ListVariant::VerticalList => html! {
        <>
<ul class="divide-y divide-gray-200" role="list">
            {
                for props.children.iter().enumerate().map(|(i,c)| { 
                    if i != props.children.len() - 1 {
                        html! {
                            <> 
                                { c }
                                <Divider/>
                            </>
                        }
                    }
                    else {
                        c
                    }
                }) 
            }
</ul>
        </>
        },
        ListVariant::TwoColList => html! {
            <>
            <dl class="grid grid-cols-1 gap-x-4 gap-y-8 sm:grid-cols-2">
                {
                    for props.children.iter()
                }
            </dl>
            </>
        },
    }
}
