use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct DataListItemWithActionProps {
    pub label: String,
    pub value: String,
    pub button_label: String,
    pub onaction: Callback<MouseEvent>
}


#[function_component(DataListItemWithAction)]
pub fn data_list_item_with_action(props: &DataListItemWithActionProps) -> Html {
    html! {
        <>              
  <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
   <dt class="text-sm font-medium text-gray-500">
    {&props.label}
   </dt>
   <dd class="mt-1 flex text-sm text-gray-900 sm:mt-0 sm:col-span-2">
    <span class="flex-grow">
     {&props.value}
    </span>
    <span class="ml-4 flex-shrink-0">
     <button onclick={&props.onaction} class="bg-white rounded-md font-medium text-indigo-600 hover:text-indigo-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500" type="button">
      {&props.button_label}
     </button>
    </span>
   </dd>
  </div>
        </>
    }
}

