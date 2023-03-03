mod api;
mod components;
mod store;

use components::{
    alert::{AlertComponent, Props as AlertProps},
    feedback_form::FeedbackForm,
    feedback_list::FeedbackList,
    feedback_stats::FeedbackStats,
};
use store::Store;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component]
fn App() -> Html {
    let (store, _) = use_store::<Store>();
    let message = store.alert_input.alert_message.clone();
    let show_alert = store.alert_input.show_alert;
    let loading = &store.loading;

    let alert_props = AlertProps {
        message,
        delay_ms: 5000,
    };
    html! {
        <>
            if show_alert {
                    <AlertComponent
                        message={alert_props.message}
                        delay_ms={alert_props.delay_ms}
                     />
                }
            <main class="md:container mt-24 px-5">
                <FeedbackForm />
                <FeedbackStats />
                <FeedbackList />
            </main>
            if *loading {
                <div
                    class="fixed top-5 left-5 inline-block h-8 w-8 animate-spin rounded-full border-4 border-solid border-yellow-400 border-r-transparent align-[-0.125em] text-warning motion-reduce:animate-[spin_1.5s_linear_infinite]"
                    role="status">
                    <span
                    class="!absolute !-m-px !h-px !w-px !overflow-hidden !whitespace-nowrap !border-0 !p-0 ![clip:rect(0,0,0,0)]"
                    >{"Loading..."}</span
                >
                </div>
            }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
