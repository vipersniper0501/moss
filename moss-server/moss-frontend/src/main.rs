use yew::prelude::*;


#[function_component(App)]
fn app() -> Html {
    

    html! {
        <>
            <div class="nav-horizontal">
                <ul>
                    <li><a class={"active"} href={"/"}>{"Home"}</a></li>
                    <li><a href={"/"}>{"Configurations"}</a></li>
                    <li style={"float: right; display: block; \
                                color: white; text-align: center; \
                                padding: 14px 16px; text-decoration: none;"}>
                                {"Moss Admin Dashboard"}</li>
                </ul>
            </div>
            <div class={"content"}>
                <h1>{"Moss"}</h1>
                <div class={"cards-column"}>
                    <div class={"card"}>
                        <p>{"Card 1"}</p>
                    </div>
                    <div class={"card"}>
                        <p>{"Card 2"}</p>
                    </div>
                    <div class={"card"}>
                        <p>{"Card 3"}</p>
                    </div>

                </div>

                <div class={"teams-column"}>

                    <p>{"Test"}</p>
                </div>
            </div>
        </>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
