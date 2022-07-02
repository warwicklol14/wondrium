use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ListHeaderProps {
    pub title: String,
    pub subtitle: String,
}

#[function_component(ListHeader)]
pub fn list_header(props: &ListHeaderProps) -> Html {
    html! {
           <>
    <div class="py-5">
     <h3 class="text-lg leading-6 font-medium text-gray-900">
      {&props.title}
     </h3>
     <p class="mt-1 max-w-2xl text-sm text-gray-500">
      {&props.subtitle}
     </p>
    </div>
           </>
       }
}
