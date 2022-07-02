use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputBarProps {
    pub onchange: Callback<Event>,
}

#[function_component(InputBar)]
pub fn input_bar(props: &InputBarProps) -> Html {
    html! {
    <div class="flex flex-col">
     <div class="sticky top-0 z-10 flex-shrink-0 flex h-16 bg-white shadow">
      <div class="flex-1 flex justify-between">
       <div class="flex-1 flex">
        <div class="w-full flex md:ml-0">
         <label class="sr-only" for="link-field">
          {"Enter the link here"}
         </label>
         <div class="relative w-full text-gray-400 focus-within:text-gray-600">
          <input class="block w-full h-full pl-8 pr-3 py-2 border-transparent text-gray-900 placeholder-gray-500 focus:outline-none focus:placeholder-gray-400 focus:ring-0 focus:border-transparent sm:text-sm" id="link-field" name="link" placeholder="Enter the link here" type="text" onchange={&props.onchange} />
         </div>
        </div>
       </div>
      </div>
     </div>
    </div>
       }
}
