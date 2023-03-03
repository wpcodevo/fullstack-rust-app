use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub selected: u8,
    #[prop_or_default]
    pub onchange: Callback<u8>,
}

#[function_component]
pub fn Rating(props: &Props) -> Html {
    let selected = props.selected;

    let onchange = props.onchange.clone();
    let on_input_change = Callback::from(move |event: Event| {
        let target = event.target().unwrap();
        let value = target.unchecked_into::<HtmlInputElement>().value();
        let selected = value.parse::<u8>().unwrap();
        onchange.emit(selected)
    });

    html! {
        <ul class="list-none flex items-center justify-around my-7">
            { for (1..=10).map(|i| {
                let label = i.to_string();
                let id = format!("num{}", i);

                html! {
                    <li class={format!("relative bg-gray-200 w-14 h-14 p-3 text-center rounded-full border-gray-300 border-2 transition duration-300 {}",
                        if selected == i { "bg-pink-500 text-white" } else { "" },
                    )}>
                        <input type="radio" class="opacity-0" id={id.clone()} name="rating" value={Some(i.to_string())} checked={selected == i} onchange={on_input_change.clone()} />
                        <label for={id} class="absolute w-full h-full flex items-center justify-center rounded-full top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 cursor-pointer hover:bg-pink-500 hover:text-white transition duration-300">{ label }</label>
                    </li>
                }
            })}
        </ul>
    }
}
