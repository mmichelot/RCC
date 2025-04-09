use leptos::prelude::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="logo">
            <img src="/images/RCC_logo.png" alt="RCC Logo"  height="120"/>
        </div>
    }
}
