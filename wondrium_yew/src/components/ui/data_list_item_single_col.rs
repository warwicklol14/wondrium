use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DataListItemSingleColProps {
    pub label: String,
    pub value: String
}


#[function_component(DataListItemSingleCol)]
pub fn data_list_item_single_col(props: &DataListItemSingleColProps) -> Html {
    html! {
        <>
   <div class="sm:col-span-2">
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
