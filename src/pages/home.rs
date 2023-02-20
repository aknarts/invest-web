use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! (
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <h1 class="title is-1">{ "Welcome..." }</h1>
                </div>
            </div>

            <div class="tile is-parent container">
                { view_info_tiles() }
            </div>
        </div>
    )
}

fn view_info_tiles() -> Html {
    html! (
        <>
            <div class="tile is-parent">
                <div class="tile is-child box">
                    <p class="title">{ "What we do?" }</p>
                    <p class="subtitle">{ "Everything you need to know!" }</p>

                    <div class="content">
                        {r#"
                            Lorem Ipsum
                            "#}
                    </div>
                </div>
            </div>

            <div class="tile is-parent">
                <div class="tile is-child box">
                    <p class="title">{ "Who are we?" }</p>

                    <div class="content">
                        {r#"
                            Lorem Ipsum
                            "#}
                    </div>
                </div>
            </div>
        </>
    )
}
