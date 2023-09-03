use gloo_net::http::Request;
use yew::prelude::*;

use moss_lib::Team;


#[derive(Properties, PartialEq)]
struct GetTeamsProps {
    backend_url: String
}

async fn get_teams_async(backend_url: String) -> Vec<Team> {
    let teams: Vec<Team> = match Request::get(backend_url.as_str()).send().await {
        Ok(response) => {
            if response.status() == 200 {
                match response.json::<Vec<Team>>().await {
                    Ok(teams) => {
                        teams
                        // html! {
                            // <>
                                // <div class={"team"}>
                                    // { for teams.iter().map(|team|
                                        // html! {
                                            // <p>{&team.name}</p>
                                        // })}
                                // </div>
                            // </>
                        // }
                    }
                    Err(_e) => {
                        todo!();
                        // return html! {
                            // <p>{"Failed to parse JSON response"}</p>
                        // }
                    }
                }
            } else {
                todo!();
                // return html! {
                    // <p>{"Failed to fetch data from the server"}</p>
                // }
            }

        }
        Err(_e) => {
            todo!();
            // return html! { <p>{"Failed to send request"}</p> };
        }
    };

    teams
}

fn get_teams(GetTeamsProps {backend_url}: &GetTeamsProps) -> Html {

    let mut url = backend_url.clone();
    let call: &str = "get_teams";

    url.push_str(call);


    let teams = use_state(|| vec![]);

    // wasm_bindgen_futures::spawn_local(async move {
        // let fetched_teams: Html = match Request::get(call).send().await {
            // Ok(response) => {
                // if response.status() == 200 {
                    // match response.json::<Vec<Team>>().await {
                        // Ok(teams) => {
                            // html! {
                                // <>
                                    // <div class={"team"}>
                                        // { for teams.iter().map(|team|
                                            // html! {
                                                // <p>{&team.name}</p>
                                            // })}
                                    // </div>
                                // </>
                            // }
                        // }
                        // Err(e) => {
                            // return html! {
                                // <p>{"Failed to parse JSON response"}</p>
                            // }
                        // }
                    // }
                // } else {
                    // return html! {
                        // <p>{"Failed to fetch data from the server"}</p>
                    // }
                // }

            // }
            // Err(e) => {
                // return html! { <p>{"Failed to send request"}</p> };
            // }
        // };

        // teams = fetched_teams;

    // });

    html! {
        <>
        </>
    }

}

#[function_component(App)]
fn app() -> Html {
    let database_url = match option_env!("DATABASE_URL") {
        Some(x) => x,
        None => {
            return html! {
                <>
                    <p>{"Missing DATABASE_URL. Start server with the following call:"}</p>
                    <p>{"DATABASE_URL=url_goes_here BACKEND_URL=url_goes_here trunk serve"}</p>
                </>
            };
        
        }
    };
    let backend_url = match option_env!("BACKEND_URL") {
        Some(x) => x,
        None => {
            return html! {
                <>
                    <p>{"Missing BACKEND_URL. Start server with the following call:"}</p>
                    <p>{"DATABASE_URL=url_goes_here BACKEND_URL=url_goes_here trunk serve"}</p>
                </>
            };
        
        }
    };

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

                    // Will be replaced with a get_teams function to generate
                    // these
                    <div class={"team"}>
                        <p>{"Test"}</p>
                    </div>
                </div>
            </div>
        </>
    }
}


fn main() {
    // dotenv().ok()/* ; */
    /* match dotenv() { */
        /* Ok(_v) => {} */
        /* Err(e) => { */
            /* eprintln!("Failed to load .env file: {}", e); */
        /* } */
    /* } */

    // let database_url = std::env::var("DATABASE_URL")
        // .expect("Could not find mysql DATABASE_URL in .env file \
            // (DATABASE_URL=\"mydatabase_url_here\")");
    // let backend_url = std::env::var("BACKEND_URL")
        // .expect("Could not find mysql BACKEND_URL in .env file \
            // (BACKEND_URL=\"mybackend_url_here\")");


    yew::Renderer::<App>::new().render();
}
