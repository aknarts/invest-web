mod list;
mod modal;

use crate::pages::admin::users::list::UserList;
use yew::prelude::*;
use yew_hooks::use_counter;

#[function_component(Users)]
pub fn users() -> Html {
    let counter = use_counter(0);
    let count = *counter;

    let fallback = html!(
        <div class="d-flex justify-content-center">
            <span class="spinner-border text-secondary" role="status">
              <span class="sr-only">{"Loading..."}</span>
            </span>
        </div>
    );

    html!(
        <section class="grid flex-fill border-end border-start border-bottom">
            <Suspense {fallback}>
                <UserList counter={counter} key={count}/>
            </Suspense>
        </section>
    )
}
