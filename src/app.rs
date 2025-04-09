use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{components::{Route, Router, Routes}, StaticSegment};

use crate::components::layout::Layout;
use crate::pages::{
    home::HomePage,
    informations::InformationsPage,
    partenaires::PartenairesPage,
    news::NewsPage,
    contact::ContactPage,
    a_propos::AProposPage,
    not_found::NotFoundPage,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rcc.css"/>

        // sets the document title
        <Title text="RCC - Accueil"/>

        // content for this welcome page
        <Router>
            <Layout>
                <Routes fallback=|| view! { <NotFoundPage/> }>
                    <Route path=StaticSegment("") view=|| view! { <HomePage/> }/>
                    <Route path=StaticSegment("informations") view=|| view! { <InformationsPage/> }/>
                    <Route path=StaticSegment("partenaires") view=|| view! { <PartenairesPage/> }/>
                    <Route path=StaticSegment("news") view=|| view! { <NewsPage/> }/>
                    <Route path=StaticSegment("contact") view=|| view! { <ContactPage/> }/>
                    <Route path=StaticSegment("a-propos") view=|| view! { <AProposPage/> }/>
                </Routes>
            </Layout>
        </Router>
    }
}
