use yew::prelude::*;
use yewdux::prelude::*;

use crate::store::Store;

#[function_component]
pub fn FeedbackStats() -> Html {
    let (store, _) = use_store::<Store>();
    let count = store.feedbacks.len();
    let sum: u32 = store.feedbacks.iter().map(|f| u32::from(f.rating)).sum();

    let average = if count > 0 {
        format!("{:.2}", sum as f32 / count as f32)
    } else {
        "0.0".to_string()
    };

    html! {
        <div class="flex justify-between items-center mb-11">
            <h4 class="text-white">{count} {" "} {"Reviews"}</h4>
            <h4 class="text-white">{"Ratings Average: "} {average}</h4>
        </div>
    }
}
