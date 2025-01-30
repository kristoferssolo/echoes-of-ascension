use leptos::{ev::SubmitEvent, prelude::*};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormData {
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseData {
    username: String,
    code: String,
}

#[server]
async fn register(payload: FormData) -> Result<ResponseData, ServerFnError> {
    let client = Client::new();
    let response = client
        .post("http://localhost:8000/api/v1/register")
        .json(&payload)
        .send()
        .await?;

    match response.status() {
        StatusCode::CREATED => {
            let response_data = response.json::<ResponseData>().await?;
            Ok(response_data)
        }
        status => {
            let error_msg = response.text().await?;
            Err(ServerFnError::ServerError(format!(
                "Registration failed: {} - {}",
                status, error_msg
            )))
        }
    }
}

#[component]
pub fn RegisterForm() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let register_action = Action::new(|input: &FormData| {
        let input = input.clone();
        async move { register(input).await }
    });
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let form_data = FormData {
            username: username.get(),
        };
        register_action.dispatch(form_data);
    };
    let is_submitting = move || register_action.pending().get();

    view! {
        <form on:submit=on_submit>
            <div>
                <label>"Username"</label>
                <input
                    type="text"
                    on:input=move |ev| username.set(event_target_value(&ev))
                    prop:value=username
                />
            </div>
            <button type="submit" disabled=is_submitting>
                {move || { if is_submitting() { "Registering..." } else { "Register" } }}
            </button>
            <ErrorBoundary fallback=move |errors| {
                view! {
                    <div class="error-container">
                        <p class="error-title">"Registration Error:"</p>
                        <ul class="error-list">
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| {
                                        view! { <li class="error-item">{e.to_string()}</li> }
                                    })
                                    .collect_view()
                            }}
                        </ul>
                    </div>
                }
            }>
                {move || {
                    register_action
                        .value()
                        .get()
                        .map(|result| {
                            match result {
                                Ok(response) => {
                                    view! {
                                        <div class="success-container">
                                            <p class="success-message">"Registration successful!"</p>
                                            <div class="registration-details">
                                                <p>"Username: " {response.username}</p>
                                                <p>"Your Code: " {response.code}</p>
                                                <p class="code-notice">
                                                    "Please save this code. You will need it to login."
                                                </p>
                                            </div>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(e) => view! { <span>{e.to_string()}</span> }.into_any(),
                            }
                        })
                }}
            </ErrorBoundary>
        </form>
    }
}
