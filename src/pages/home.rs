use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">

            <section class="hero">
                <h1>Bienvenue sur RCC</h1>
                <img src="/images/RCC_logo.png" alt="Rugby Club Cadaujacais Logo" class="club-logo"/>
            </section>

            <section class="social-links">
                <h2>Retrouvez-nous</h2>
                <div class="social-icons">
                    <a href="mailto:edr.cadaujac33@gmail.com" class="social-link email-link">
                        <span class="icon">{"✉️"}</span>
                        <span class="link-text">edr.cadaujac33@gmail.com</span>
                    </a>
                    <a href="https://www.instagram.com/edr_rccadaujac/" target="_blank" class="social-link instagram-link">
                        <span class="icon">{"📸"}</span>
                        <span class="link-text">Instagram</span>
                    </a>
                    <a href="https://www.facebook.com/RugbyClubCadaujacais/" target="_blank" class="social-link facebook-link">
                        <span class="icon">{"👍"}</span>
                        <span class="link-text">Facebook</span>
                    </a>
                </div>
            </section>
        </div>
    }
}
