use leptos::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="not-found-page">
            <h1>"404 - Page non trouvée"</h1>
            <p>"La page que vous recherchez n'existe pas."</p>
            <a href="/" class="home-link">"Retour à l'accueil"</a>
        </div>
    }
}
