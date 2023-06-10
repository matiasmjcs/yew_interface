use yew::prelude::*;

#[function_component]
fn App() -> Html {

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
  
      html! {
        <div class={classes!("flex", "flex-col", "justify-center" ,"items-center" ,"bg-slate-950", "h-50")}>
           <h1 class={classes!("text-white", "font-bold ")}>{"yew rust"}</h1> 
           <p class={classes!("text-white", "font-bold ")}>{ "Hello, world!" }</p>
           <ul>
           <li class={classes!("text-white", "font-bold ")}> { "cafesito" }</li>
           <li class={classes!("text-white", "font-bold ")}> { "totolino" }</li>
           </ul>
           <button {onclick} class={classes!("bg-white", "font-bold", "p-2")}>{ "Increment" }</button>
           <p class={classes!("text-white", "font-bold ")}> { *counter }</p>
        </div>       
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}