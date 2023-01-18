use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlInputElement, InputEvent};
use yew::virtual_dom::VNode;
use yew::{
    function_component, html, use_state, Callback, Html, MouseEvent, Properties, UseStateHandle,
};

use crate::youtube::search_youtube;
mod api;
mod youtube;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let video: UseStateHandle<Option<Video>> = use_state(|| None);

    let on_search: Callback<String> = {
        let video: UseStateHandle<Option<Video>> = video.clone();
        Callback::from(move |text| {
            let video: UseStateHandle<Option<Video>> = video.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match search_youtube(text).await {
                    Ok(video_item) => video.set(Some(Video {
                        id: video_item.id.video_id,
                        name: video_item.snippet.title,
                    })),
                    Err(e) => {
                        web_sys::console::log_1(&e.to_string().into());
                    }
                }
            });
        })
    };

    let video_section: VNode = match (*video).clone() {
        Some(video) => html! {
            <VideoSection name={video.name} id={video.id} />
        },
        None => html! {
            <p>{"Non hai video"}</p>
        },
    };
    html! {
    <section class="section">
       <div class="container">
           <div class="columns is-vcentered">
                <div class="column auto">
                   <InputSection on_search={on_search} ></InputSection>
               </div>
           <div class="column">
            {video_section}
           </div>
    </div>
    </div>
    </section>
           }
}

#[function_component(InputSection)]
fn input_section(props: &InputSectionProps) -> Html {
    let text: UseStateHandle<String> = use_state(|| String::new());

    let handle_input = {
        let text = text.clone();
        Callback::from(move |input_event| {
            let input_text = get_value_from_input_event(input_event);
            text.set(input_text);
        })
    };

    let on_search_pressed: Callback<MouseEvent> = {
        let on_search = props.on_search.clone();
        Callback::from(move |_| on_search.emit(text.to_string()))
    };

    html! {
       <div>
       <h1 class="title">
       {"Buscador de YouTube"}
      </h1>
      <div class="field">
      <label class="label">{"Insire unha palabra"}</label>
      <div class="control">
      <input class="input is-primary" type="text" oninput={handle_input}/>
      </div>
      </div>
      <div class="control">
      <button onclick={on_search_pressed} class="button is-primary">{"Buscar"}</button>
    </div>
    </div>
     }
}

#[derive(Properties, PartialEq)]
struct InputSectionProps {
    on_search: Callback<String>,
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps {
    id: String,
    name: String,
}

#[function_component(VideoSection)]
fn video_section(props: &VideoSectionProps) -> Html {
    let yt_url: String = format!("https://www.youtube.com/embed/{}", props.id);
    html! {
        <div class="box auto">
        <iframe width="560" height="315" src={yt_url}></iframe>
    </div>

    }
}

#[derive(Clone)]
struct Video {
    id: String,
    name: String,
}

fn get_value_from_input_event(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}
