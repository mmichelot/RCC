# Leptos: Guide Complet du Framework Web Rust

## Introduction

Leptos est un framework web pour Rust qui permet de créer des applications web interactives en utilisant un modèle de réactivité fine. Ce document synthétise les concepts clés et les fonctionnalités de Leptos pour servir de référence complète.

## Structure du Framework

Leptos propose deux modèles principaux de développement:

1. **Client-Side Rendering (CSR)** - Utilisant [Trunk](https://trunkrs.dev/) pour compiler l'application Rust en WebAssembly
2. **Server-Side Rendering (SSR)** - Utilisant [`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos) pour créer des applications full-stack

L'architecture de Leptos repose sur un système réactif fin, permettant des mises à jour ciblées du DOM pour minimiser les modifications.

## Installation et Démarrage

### Prérequis
- Rust installé et à jour
- Cible WebAssembly ajoutée: `rustup target add wasm32-unknown-unknown`

### Configuration pour CSR avec Trunk
```bash
cargo install trunk
cargo init leptos-tutorial
cd leptos-tutorial
cargo add leptos --features=csr
```

### Configuration pour SSR avec cargo-leptos
```bash
cargo install --locked cargo-leptos
cargo leptos new --git https://github.com/leptos-rs/start-axum
# ou
cargo leptos new --git https://github.com/leptos-rs/start-actix
```

### Amélioration de l'expérience développeur
1. Ajouter `console_error_panic_hook` pour de meilleurs messages d'erreur
2. Configurer l'autocomplétion dans les macros avec les paramètres adaptés à votre éditeur
3. Installer `leptosfmt` pour formater le code dans les macros `view!`
4. Utiliser `--cfg=erase_components` pendant le développement pour accélérer les temps de compilation

## Composants de Base

### Structure d'un Composant
```rust
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button on:click=move |_| *set_count.write() += 1>
            "Click me: " {count}
        </button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
```

### Éléments Clés
- Macro `#[component]` pour définir un composant
- Fonctions qui retournent `impl IntoView`
- Macros `view!` pour créer l'UI avec une syntaxe similaire à JSX
- Fermetures réactives dans les `{}` pour les valeurs dynamiques

## Système de Réactivité

### Signaux
Les signaux sont la base de la réactivité dans Leptos:

```rust
// Création d'un signal avec une valeur initiale
let (count, set_count) = signal(0);

// Lecture de la valeur
count.get() // ou count() avec la feature nightly
count.read() // Lecture sans clonage
count.with(|n| /* faire quelque chose avec la référence */)

// Modification de la valeur
set_count.set(42) // Remplacement complet
*set_count.write() += 1 // Mise à jour avec référence mutable
set_count.update(|n| *n += 1) // Mise à jour via fonction
```

### Signaux Dérivés
Fonctions qui accèdent à d'autres signaux et créent des calculs réactifs:

```rust
let double_count = move || count.get() * 2;
```

### Memo
Un calcul réactif mis en cache qui ne s'exécute que lorsque ses dépendances changent:

```rust
let expensive_calculation = Memo::new(move |_| {
    // Calcul coûteux qui dépend de count
    count.get() * 1000
});
```

### Effets
Exécutent du code lorsque des signaux changent:

```rust
Effect::new(move |_| {
    logging::log!("Count changed to: {}", count.get());
});
```

## Construction d'Interface Utilisateur

### Attributs Dynamiques
```rust
view! {
    <button
        class:active=move || is_active.get()
        style:color=move || if is_error.get() { "red" } else { "blue" }
        style=("--count", move || count.get().to_string())
        prop:disabled=move || is_disabled.get()
    >
        "Click Me"
    </button>
}
```

### Composants et Props
```rust
#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max={max} value=progress></progress>
    }
}

// Utilisation
view! {
    <ProgressBar max=50 progress=count/>
}
```

### Itération
Pour des listes statiques:
```rust
let values = vec![0, 1, 2];
view! {
    <ul>
        {values.into_iter()
            .map(|n| view! { <li>{n}</li>})
            .collect_view()}
    </ul>
}
```

Pour des listes dynamiques:
```rust
view! {
    <For
        each=move || todos.get()
        key=|todo| todo.id
        children=move |todo| view! { <TodoItem todo/> }
    />
}
```

### Contrôle de Flux
```rust
// Conditionnels avec if
view! {
    <p>
        {move || if count.get() % 2 == 0 {
            "Even"
        } else {
            "Odd"
        }}
    </p>
}

// Avec Show
view! {
    <Show
        when=move || count.get() > 5
        fallback=|| view! { <p>"Too small"</p> }
    >
        <p>"Big enough"</p>
    </Show>
}
```

### Gestion des Erreurs
```rust
view! {
    <ErrorBoundary
        fallback=|errors| view! {
            <div class="error">
                <p>"Error occurred: "</p>
                <ul>
                    {move || errors.get().into_iter()
                        .map(|(_, e)| view! { <li>{e.to_string()}</li>})
                        .collect::<Vec<_>>()
                    }
                </ul>
            </div>
        }
    >
        {value} // Result<T, E> qui peut déclencher le fallback
    </ErrorBoundary>
}
```

### Formulaires et Entrées

#### Inputs Contrôlés
```rust
let (name, set_name) = signal(String::new());

view! {
    <input type="text"
        on:input:target=move |ev| set_name.set(ev.target().value())
        prop:value=name
    />
    <p>"Name: " {name}</p>
}
```

Avec la syntaxe `bind:`:
```rust
view! {
    <input type="text" bind:value=(name, set_name) />
    <input type="checkbox" bind:checked=spam_me />
}
```

#### Inputs Non-Contrôlés
```rust
let input_ref = NodeRef::<html::Input>::new();

let on_submit = move |ev: SubmitEvent| {
    ev.prevent_default();
    let value = input_ref.get().expect("input exists").value();
    // Utiliser value...
};

view! {
    <form on:submit=on_submit>
        <input type="text" node_ref=input_ref />
        <input type="submit" value="Submit" />
    </form>
}
```

## Communication Parent-Enfant

### 1. Passage d'un WriteSignal
```rust
#[component]
fn Parent() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! { <Child setter=set_count /> }
}

#[component]
fn Child(setter: WriteSignal<i32>) -> impl IntoView {
    view! { <button on:click=move |_| setter.update(|n| *n += 1)>"Increment"</button> }
}
```

### 2. Utilisation d'un Callback
```rust
#[component]
fn Parent() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! { <Child on_click=move |_| set_count.update(|n| *n += 1) /> }
}

#[component]
fn Child(on_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! { <button on:click=on_click>"Click Me"</button> }
}
```

### 3. Écouteurs d'Événements Directs
```rust
#[component]
fn Parent() -> impl IntoView {
    let (count, set_count) = signal(0);
    view! { <Child on:click=move |_| set_count.update(|n| *n += 1) /> }
}

#[component]
fn Child() -> impl IntoView {
    view! { <button>"Click Me"</button> }
}
```

### 4. Utilisation du Contexte
```rust
#[component]
fn Parent() -> impl IntoView {
    let (count, set_count) = signal(0);
    provide_context(set_count);
    view! { <Child /> }
}

#[component]
fn Child() -> impl IntoView {
    let set_count = use_context::<WriteSignal<i32>>().expect("context provided");
    view! { <button on:click=move |_| set_count.update(|n| *n += 1)>"Increment"</button> }
}
```

## Enfants de Composants

```rust
#[component]
fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout">
            <header>"My App"</header>
            <main>{children()}</main>
            <footer>"Copyright 2023"</footer>
        </div>
    }
}

// Utilisation
view! {
    <Layout>
        <p>"Content goes here"</p>
    </Layout>
}
```

Manipulation des enfants:
```rust
#[component]
fn List(children: ChildrenFragment) -> impl IntoView {
    let items = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view! { <ul>{items}</ul> }
}
```

## Gestion de l'État Global

### 1. URL comme État Global
Le routeur utilise l'URL comme source d'état global, accessible depuis n'importe quel composant.

### 2. Passage de Signaux via Contexte
```rust
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    provide_context(count);

    view! {
        <SetterButton set_count/>
        <Consumer/>
    }
}

#[component]
fn Consumer() -> impl IntoView {
    let count = use_context::<ReadSignal<i32>>().expect("count in context");
    view! { <p>"Count: " {count}</p> }
}
```

### 3. Création d'un Store Global
```rust
#[derive(Clone, Debug, Default, Store)]
struct GlobalState {
    count: i32,
    name: String,
}

#[component]
fn App() -> impl IntoView {
    provide_context(Store::new(GlobalState::default()));
    // ...
}

#[component]
fn Counter() -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();
    let count = state.count();

    view! {
        <button on:click=move |_| *count.write() += 1>
            "Increment Global Count"
        </button>
        <span>"Count: " {move || count.get()}</span>
    }
}
```

## Gestion des Tâches Asynchrones

### Resources
```rust
// Resource locale (client uniquement)
let async_data = LocalResource::new(move || load_data(count.get()));

// Resource (client et serveur)
let async_data = Resource::new(
    move || count.get(), // source
    |count| load_data(count) // fetcher
);

// Accès aux données
view! {
    <p>
        {move || async_data.get().map(|data| format!("Data: {data}"))
            .unwrap_or_else(|| "Loading...".into())}
    </p>
}
```

### Suspense
```rust
view! {
    <Suspense fallback=move || view! { <p>"Loading..."</p> }>
        {move || Suspend::new(async move {
            let data = async_data.await;
            view! { <DataDisplay data/> }
        })}
    </Suspense>
}
```

### Transition
```rust
view! {
    <Transition
        fallback=move || view! { <p>"Loading initial data..."</p> }
        set_pending
    >
        <p>{move || user_data.read().map(|d| d.name)}</p>
    </Transition>
}
```

### Actions
```rust
let add_todo = Action::new(|input: &String| {
    let input = input.to_owned();
    async move { add_todo_request(&input).await }
});

// Utilisation dans un formulaire
view! {
    <button on:click=move |_| add_todo.dispatch("Nouvelle tâche".to_string())>
        "Ajouter"
    </button>
    <p>{move || add_todo.pending().get().then_some("Loading...")}</p>
}
```

## Routage

### Définition des Routes
```rust
view! {
    <Router>
        <Routes fallback=|| "Page not found">
            <Route path="/" view=Home/>
            <Route path="/users" view=Users/>
            <Route path="/users/:id" view=UserProfile/>
            <ParentRoute path="/contacts" view=ContactList>
                <Route path=":id" view=ContactInfo/>
                <Route path="" view=NoContactSelected/>
            </ParentRoute>
        </Routes>
    </Router>
}
```

### Navigation
```rust
view! {
    <nav>
        <A href="/">Home</A>
        <A href="/users">Users</A>
    </nav>
}
```

### Params et Queries
```rust
#[derive(Params)]
struct UserParams {
    id: Option<usize>,
}

#[component]
fn UserProfile() -> impl IntoView {
    let params = use_params::<UserParams>();
    let id = move || params.read().unwrap_or_default().id.unwrap_or_default();

    view! { <p>"User ID: " {id}</p> }
}
```

### Formulaires et Navigation
```rust
view! {
    <Form method="GET" action="/search">
        <input type="search" name="q" value=search
            oninput="this.form.requestSubmit()"
        />
    </Form>
}
```

## Server-Side Rendering (SSR)

### Modes de Rendu SSR
1. **Synchrone** - Envoie une structure HTML sans attendre les données async
2. **Async** - Attend que toutes les données soient chargées pour envoyer le HTML
3. **Streaming en Ordre** - Envoie le HTML par morceaux en suivant l'ordre du document
4. **Streaming Hors-Ordre** - Envoie d'abord le squelette HTML puis remplace les sections suspendues quand les données sont prêtes
5. **Streaming Partiellement Bloqué** - Bloque sur certaines resources critiques mais stream les autres

```rust
<Routes>
    <Route path="" view=HomePage/>
    <Route path="/post/:id" view=Post ssr=SsrMode::Async/>
</Routes>
```

### Fonctions Serveur
```rust
#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    match sqlx::query("INSERT INTO todos (title, completed) VALUES ($1, false)")
        .bind(title)
        .execute(&mut conn)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

// Utilisation côté client
view! {
    <button on:click=move |_| {
        spawn_local(async {
            add_todo("Nouvelle tâche".to_string()).await;
        });
    }>
        "Ajouter tâche"
    </button>
}
```

### Extracteurs
```rust
#[server]
pub async fn user_data() -> Result<String, ServerFnError> {
    use axum::{extract::Query, http::Method};
    use leptos_axum::extract;

    let (method, query): (Method, Query<UserQuery>) = extract().await?;
    // Traitement des données...
    Ok(format!("Method: {method:?}, Query: {query:?}"))
}
```

### Réponses et Redirections
```rust
#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    // Vérification d'authentification...
    if auth_success {
        // Redirection après succès
        leptos_axum::redirect("/dashboard");
        Ok(())
    } else {
        Err(ServerFnError::ServerError("Login failed".into()))
    }
}
```

## Intégration JavaScript et Web APIs

### Utilisation de NodeRef pour Accéder aux Éléments DOM
```rust
let input_ref = NodeRef::<html::Input>::new();

view! {
    <input type="text" node_ref=input_ref />
    <button on:click=move |_| {
        if let Some(input) = input_ref.get() {
            logging::log!("Value: {}", input.value());
        }
    }>
        "Get Value"
    </button>
}
```

### Utilisation de web_sys
```rust
#[component]
fn AudioPlayer() -> impl IntoView {
    let audio_ref = NodeRef::<html::Audio>::new();

    let play = move |_| {
        if let Some(audio) = audio_ref.get() {
            let _ = audio.play();
        }
    };

    view! {
        <audio node_ref=audio_ref src="/music.mp3"></audio>
        <button on:click=play>"Play"</button>
    }
}
```

## Améliorations Progressives

### ActionForm pour Formulaires Robustes
```rust
#[server]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    // Traitement serveur...
    Ok(())
}

#[component]
fn AddTodoForm() -> impl IntoView {
    let add_todo = ServerAction::<AddTodo>::new();

    view! {
        <ActionForm action=add_todo>
            <label>
                "Nouvelle tâche"
                <input type="text" name="title"/>
            </label>
            <input type="submit" value="Ajouter"/>
        </ActionForm>
    }
}
```

## Architecture Islands

```rust
#[island]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <button on:click=move |_| *set_count.write() += 1>
            "Click me: " {count}
        </button>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"My Static Content"</h1>
        <p>"This is server-rendered only."</p>
        <Counter/>  // Sera hydratable
    }
}
```

## Tests

### Test de la Logique Métier
```rust
pub struct Todos(Vec<Todo>);

impl Todos {
    pub fn num_remaining(&self) -> usize {
        self.0.iter().filter(|todo| !todo.completed).count()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_remaining() {
        let todos = Todos(vec![
            Todo { completed: true, ... },
            Todo { completed: false, ... },
        ]);
        assert_eq!(todos.num_remaining(), 1);
    }
}
```

### Tests End-to-End
Utilisation d'outils comme `wasm-bindgen-test`, Playwright, ou Cucumber pour tester l'interface.

## Déploiement

### Applications CSR
```bash
trunk build --release
# Déployer le contenu du dossier 'dist'
```

### Applications SSR
```dockerfile
FROM rustlang/rust:nightly-bullseye as builder

# Installation des outils
RUN cargo install cargo-leptos --version 0.2.*
RUN rustup target add wasm32-unknown-unknown

# Copie des sources
WORKDIR /app
COPY . .

# Build
RUN cargo leptos build --release

FROM debian:bookworm-slim as runtime
WORKDIR /app
COPY --from=builder /app/target/release/my_app /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

# Configuration
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

CMD ["/app/my_app"]
```

## Optimisation

### Taille du Binaire WASM
```toml
[profile.wasm-release]
inherits = "release"
opt-level = 'z'
lto = true
codegen-units = 1

[package.metadata.leptos]
lib-profile-release = "wasm-release"
```

## Ressources Utiles

- **Documentation officielle**: [docs.rs/leptos](https://docs.rs/leptos/latest/leptos/)
- **Repo GitHub**: [github.com/leptos-rs/leptos](https://github.com/leptos-rs/leptos)
- **Discord**: [discord.gg/YdRAhS7eQB](https://discord.gg/YdRAhS7eQB)
- **Crates communautaires**: [github.com/leptos-rs/awesome-leptos](https://github.com/leptos-rs/awesome-leptos)

## Bonnes Pratiques

1. **Séparer la logique** des composants pour faciliter les tests
2. **Éviter les effets** qui écrivent dans des signaux si possible
3. **Utiliser les contextes** pour éviter le "prop drilling"
4. **Choisir judicieusement entre signaux dérivés et mémos** selon la complexité du calcul
5. **Structurer les applications** avec des composants de présentation et des composants contenant la logique
6. **Préférer `<For/>` avec clés** pour les listes dynamiques
7. **Utiliser l'URL comme source d'état global** quand c'est pertinent

Ce document couvre l'essentiel de Leptos, un framework web Rust moderne et réactif qui offre une excellente expérience de développement pour créer des applications web performantes et maintenables.
