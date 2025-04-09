use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <section class="hero">
                <h1>Bienvenue sur RCC</h1>
                <p>Découvrez notre offre de services innovants.</p>
                <button class="cta-button">En savoir plus</button>
            </section>

            <section class="features">
                <h2>Nos services</h2>
                <div class="features-grid">
                    <div class="feature-card">
                        <h3>Service 1</h3>
                        <p>Description du service 1.</p>
                    </div>
                    <div class="feature-card">
                        <h3>Service 2</h3>
                        <p>Description du service 2.</p>
                    </div>
                    <div class="feature-card">
                        <h3>Service 3</h3>
                        <p>Description du service 3.</p>
                    </div>
                </div>
            </section>
        </div>
    }
}
