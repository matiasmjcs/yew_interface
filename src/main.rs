use yew::prelude::*;

#[function_component]
fn App() -> Html {
  
      html! {
        <div class="flex flex-col justify-center items-center bg-slate-950 ">
           <h1 class="text-white text-bold " >{"yew rust"}</h1> 
           <p class="text-white text-bold "> { "Hello, world!" }</p>
        </div>       
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}