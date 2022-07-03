use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DividerProps {
    #[prop_or_default]
    pub children: Children,
}


#[function_component(Divider)]
pub fn divider(props: &DividerProps) -> Html {
    html! {
        <>
<div class="relative">
 <div aria-hidden="true" class="absolute inset-0 flex items-center">
  <div class="w-full border-t border-gray-300">
  </div>
 </div>
 <div class="relative flex justify-center">
    <span class="bg-gray-50">
    { for props.children.iter() }
    </span>
 </div>
</div>
        </>
    }
}
