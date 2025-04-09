use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    let toggle_menu = move |_| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };

    view! {
        <nav class="navbar">
            <div class="navbar-container">
                <div class="navbar-logo">
                    <crate::components::logo::Logo />
                </div>

                <button class="mobile-menu-button" on:click=toggle_menu>
                    <span class="menu-icon">"☰"</span>
                </button>

                <ul class=move || {
                    if mobile_menu_open.get() {
                        "nav-links mobile-active"
                    } else {
                        "nav-links"
                    }
                }>
                    <li class="nav-link"><A href="/" >"Accueil"</A></li>
                    <li class="nav-link"><A href="/informations" >"Informations"</A></li>
                    <li class="nav-link"><A href="/partenaires" >"Partenaires"</A></li>
                    <li class="nav-link"><A href="/news" >"News"</A></li>
                    <li class="nav-link"><A href="/contact" >"Contact"</A></li>
                    <li class="nav-link"><A href="/a-propos" >"À propos"</A></li>
                </ul>
            </div>
        </nav>
    }
}
