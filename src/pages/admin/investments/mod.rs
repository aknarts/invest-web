mod cost_line;
mod costs;
mod list;
pub mod modal;
mod picture;
pub mod pictures;
mod tag;
mod tags;

use crate::pages::admin::investments::list::InvestmentsList;
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component(Investments)]
pub fn investments() -> Html {
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
                <InvestmentsList counter={counter} key={count}/>
            </Suspense>
        </section>
    )
}
