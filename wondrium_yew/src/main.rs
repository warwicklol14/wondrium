use web_sys::HtmlInputElement;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use gloo_console::log;

mod components;
mod backend;

use components::ui::InputBar;
use components::CourseContainer;

use wondrium_lib::Course;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
pub fn app() -> Html {
    let course_state = use_state_eq(|| Course::default());

    let get_course_from_backend = {
        move |course_link: String, course_state: UseStateHandle<Course>| {
            wasm_bindgen_futures::spawn_local(async move {
                if course_link.contains("wondrium.com") {
                    let course_g = backend::get_course(course_link).await;
                    course_state.set(course_g);
                    log!(JsValue::from_serde(&*course_state).unwrap());
                }
            });
        }
    };

    let onchange = {
        let course_state = course_state.clone();
        Callback::from(move |e: Event| {
            let input_el: HtmlInputElement = e.target_unchecked_into();
            let course_link = input_el.value();
            get_course_from_backend(course_link, course_state.clone());
        })
    };

    let play_lecture = {
        Callback::from(|lecture_name: String| {
            wasm_bindgen_futures::spawn_local(async move {
                backend::play_lecture(lecture_name).await;
            });
        })
    };

    if course_state.course_id != 0 {
        html! {
            <>
                <InputBar {onchange} />
                <CourseContainer {play_lecture} course={(*course_state).clone() }/>
            </>
        }
    }
    else {
        html! {
            <>
                <InputBar {onchange} />
            </>
        }
    }
}
