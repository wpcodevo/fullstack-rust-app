use yew::prelude::*;
use yewdux::prelude::*;

use super::feedback_item::FeedbackItem;
use crate::{
    api::api_fetch_feedbacks,
    store::{set_feedback_list, set_loading, set_show_alert, Store},
};

#[function_component]
pub fn FeedbackList() -> Html {
    let (store, dispatch) = use_store::<Store>();
    let feedback_list = store.feedbacks.clone();

    use_effect_with_deps(
        move |_| {
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                set_loading(true, dispatch.clone());
                let response = api_fetch_feedbacks((1, 10)).await;
                match response {
                    Ok(feedbacks) => {
                        set_loading(false, dispatch.clone());
                        set_feedback_list(feedbacks, dispatch);
                    }
                    Err(e) => {
                        set_loading(false, dispatch.clone());
                        set_show_alert(e.to_string(), dispatch);
                    }
                }
            });
        },
        (),
    );

    html! {
        <div>
            {
                feedback_list.into_iter().map(|feedback|{
                    let key = feedback.id.to_string();
                    html!{<FeedbackItem {key} feedback={feedback.clone()} />}
                }).collect::<Html>()
            }
        </div>
    }
}
