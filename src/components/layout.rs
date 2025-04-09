use leptos::children::Children;
use leptos::prelude::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout">
            <crate::components::navbar::Navbar />
            <main class="main-content">
                {children()}
            </main>
            <footer class="footer">
                <p>{"© 2023 RCC. Tous droits réservés."}</p>
            </footer>
        </div>
    }
}
