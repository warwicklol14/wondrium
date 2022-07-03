use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardContainerProps {
    #[prop_or_default]        
    pub children: Children,
}


#[function_component(CardContainer)]
pub fn card_container(props: &CardContainerProps) -> Html {
    html! {
        <>
        <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
                { for props.children.iter() }
            </div>
        </div> 
        </>
    }
}
