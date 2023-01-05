use yew::{function_component, html, Callback, Html, InputEvent, Properties};

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    html! {
    <section class="section">
       <div class="container">
           <div class="columns is-vcentered">
                <div class="column auto">
                   <InputSection></InputSection>
               </div>
           <div class="column">
                <VideoSection name="nome do video" id="dQw4w9WgXcQ"/>
           </div>
    </div>
    </div>
    </section>


           }
}

#[function_component(InputSection)]
fn input_section() -> Html {
    let handle_input: Callback<InputEvent> = Callback::from(|_| {});
    html! {
       <div>
       <h1 class="title">
       {"Buscardor de YouTube"}
      </h1>
      <div class="field">
      <label class="label">{"Insire unha palabra"}</label>
      <div class="control">
      <input class="input is-primary" type="text" oninput={handle_input}/>
      </div>
      </div>
      <div class="control">
      <button class="button is-primary">{"Buscar"}</button>
    </div>
    </div>
     }
}

#[derive(Properties, PartialEq)]
struct VideoSectionProps{
    id: String,
    name: String
}

#[function_component(VideoSection)]
fn video_section( props: &VideoSectionProps) -> Html {
   let yt_url: String = format!("https://www.youtube.com/embed/{}", props.id);
    html! {
        <div class="box auto">
        <iframe width="560" height="315" src={yt_url}></iframe>
    </div>

    }
}
