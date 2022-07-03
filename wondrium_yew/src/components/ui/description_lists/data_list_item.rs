use yew::prelude::*;

#[derive(PartialEq)]
pub enum ItemVariant {
    Horizontal,
    SingleColWide,
    TwoColWide
}

#[derive(Properties, PartialEq)]
pub struct DataListItemProps {
    pub label: String,
    pub value: String,
    pub variant: ItemVariant
}


#[function_component(DataListItem)]
pub fn data_list_item(props: &DataListItemProps) -> Html {
    match props.variant {
    ItemVariant::Horizontal => html! {
            <>
              <div class="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4">
               <dt class="text-sm font-medium text-gray-500">
                {&props.label}
               </dt>
               <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                 {&props.value}
               </dd>
              </div>
            </>
        },
    ItemVariant::SingleColWide => html! {
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
    },
    ItemVariant::TwoColWide => html! {
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
    },
    }
}
