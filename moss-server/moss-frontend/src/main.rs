use leptos::*;
use moss_lib::Team;
use gloo_net::http::Request;

// This should be a result
async fn get_teams() -> Vec<Team> {
    let teams: Vec<Team> = match Request::get("http://127.0.0.1:4224/api/v1/get_teams")
        .send().await {
            Ok(response) => {
                if response.status() == 200 {
                    match response.json::<Vec<Team>>().await {
                        Ok(teams) => {
                            teams
                        }
                        Err(_e) => {
                            todo!();
                        }
                    }
                } else {
                    todo!();
                }
            }
            Err(_e) => {
                todo!();
            }
        };

    teams
}

#[component]
fn TeamsComponent(cx: Scope) -> impl IntoView {
   
    let async_data = create_resource(cx, || (),
        |_| async move {
            get_teams().await
        });

    // let value = create_signal(cx, 0);
    // let async_data = create_resource(cx, move || (), get_teams);

    // log!("Async data: {:#?}", async_data);
    // this is the problem area
    // let team_1 = match async_data.read(cx) {
        // Some(x) => {
            // view! { cx,
                // <p>"Team: " + {move || x[0].name.clone().into_view(cx)}</p>
            // }.into_view(cx)
        // },
        // None => {
            // view! { cx,
                // <p>"Loading..."</p>
            // }.into_view(cx)
        // }
    // };


    let teams = move || {
        async_data.read(cx)
            .map(|value: Vec<Team>| value)
            // .map(|value: Vec<Team>| format!("Server returns:\n{:#?}",value))
            // .unwrap_or_else(|| "Loading...".into())
            .unwrap_or_else(|| vec![])
    };

    let teams: Vec<Team> = teams();
    // log!("Server returned: {}", teams[0].name);


    // I am so confused right now...

    view!{ cx, 
        // <p>{format!("Server returns: {:#?}", teams)}</p>
        // <p>{team_1}</p>
        <p>"Testing"</p>
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {

    view! {cx,
        <div class="nav-horizontal">
            <ul>
                <li><a class="active" href="/">"Home"</a></li>
                <li><a href="/">"Configurations"</a></li>
                <li style="float: right; display: block; \
                            color: white; text-align: center; \
                            padding: 14px 16px; text-decoration: none;">
                            "Moss Admin Dashboard"</li>
            </ul>
        </div>
        <div class="content">
            <h1>"Moss"</h1>
            <div class="cards-column">
                <div class="card">
                    <p>"Card 1"</p>
                </div>
                <div class="card">
                    <p>"Card 2"</p>
                </div>
                <div class="card">
                    <p>"Card 3"</p>
                </div>
            </div>

            <div class="teams-column">
                // Will be replaced with a get_teams function to generate
                // these
                <div class="team">
                    <TeamsComponent/>
                </div>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! { cx,
        <App/>
    })
}
