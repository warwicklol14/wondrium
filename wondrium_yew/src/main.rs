use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use gloo_console::log;

mod components;
mod backend;

use components::ui::InputBar;
use components::ui::TwoColList;

use wondrium_lib::Course;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let link = use_state(|| String::new());
    let course = use_state_eq(|| Course::default());

    let onchange = {
        let course_link = link.clone();
        let course = course.clone();
        wasm_bindgen_futures::spawn_local(async move {
            if course_link.contains("wondrium.com") {
                let course_g = backend::get_course((*course_link).clone()).await;
                course.set(course_g);
                log!(JsValue::from_serde(&*course).unwrap());
            }
        });
        let link = link.clone();
        Callback::from(move |e: Event| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            link.set(input_el.value())
        })
    };

    html! {
        <>
            <div class="max-w-8xl py-2 mx-auto px-4 sm:px-6 lg:px-8">
                <div class="max-w-4xl mx-auto">
                        <InputBar {onchange} />
                        //<MediaCollection media_link={(*link).clone()}/>
                        <p>{&course.course_name} </p>
                        <TwoColList />
                </div>
            </div>
        </>
    }
}
