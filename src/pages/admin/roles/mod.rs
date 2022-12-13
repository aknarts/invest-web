mod delete_role_modal;
mod permission;
mod role_list;
mod role_modal;

use role_list::RoleList;
use tracing::debug;
use yew::prelude::*;
use yew_hooks::use_counter;

#[function_component(Roles)]
pub fn roles() -> Html {
    let counter = use_counter(0);

    let count = *counter;
    debug!("Current count: {}", count);

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
                <RoleList counter={counter} key={count}/>
            </Suspense>
        </section>
    )
}
