use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DataListItemTwoColProps {
    pub label: String,
    pub value: String
}


#[function_component(DataListItemTwoCol)]
pub fn data_list_item_two_col(props: &DataListItemTwoColProps) -> Html {
    html! {
        <>
   <div class="sm:col-span-1">
    <dt class="text-sm font-medium text-gray-500">
     {&props.label}
    </dt>
    <dd class="mt-1 text-sm text-gray-900">
     {&props.value}
    </dd>
   </div>
        </>
    }
}
