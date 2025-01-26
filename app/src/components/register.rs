use leptos::prelude::*;

use crate::server_fn::auth::register_user;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (username, set_username) = signal(String::new());
    let register_action = Action::new(|input: &RegisterUserForm| {
        let input = input.clone();
        async move { register_user(input).await }
    });

    let response = register_action.value();
    let pending = register_action.pending();

    view! {
        <div class="container">
            <form on:submit=move |ev| {
                ev.prevent_default();
                register_action
                    .dispatch(RegisterUserForm {
                        username: username.get(),
                    });
            }>
                <h1>"Register"</h1>
                <div class="input-group">
                    <label for="username">"Username"</label>
                    <input
                        type="text"
                        id="username"
                        name="username"
                        on:input=move |ev| {
                            set_username(event_target_value(&ev));
                        }
                        prop:value=username
                    />
                    <div class="error">
                        {move || {
                            response
                                .get()
                                .and_then(|result| result.err())
                                .map(|err| err.to_string())
                        }}
                    </div>
                </div>
                <button type="submit" disabled=pending>
                    {move || {
                        response
                            .with(|r| {
                                r.as_ref()
                                    .and_then(|result| result.as_ref().err())
                                    .map(|err| err.to_string())
                            })
                    }}
                    "Register"
                </button>
            </form>
            {move || {
                response
                    .with(|r| {
                        r.as_ref()
                            .and_then(|result| result.as_ref().ok())
                            .map(|res| {
                                view! {
                                    <div class="success">
                                        <p>"Reistration successful!"</p>
                                        <p>"Your code ir: "{res.code.clone()}</p>
                                    </div>
                                }
                            })
                    })
            }}
        </div>
    }
}
