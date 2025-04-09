# RCC (Rugby Club Cadaujac) - Site Web

Ce dépôt contient le code source du site web du Rugby Club Cadaujac (RCC), développé avec Leptos, un framework Rust pour le développement web.

## Présentation du projet

Ce site web a pour but de fournir une plateforme en ligne pour le Rugby Club Cadaujac, permettant aux membres et visiteurs d'accéder aux informations du club, aux actualités, aux événements et plus encore.

## Technologies utilisées

- **Rust** - Langage de programmation principal
- **Leptos** - Framework web réactif pour Rust
- **Axum** - Framework web pour le backend
- **WebAssembly** - Pour l'exécution côté client
- **SCSS** - Pour les styles

## Structure du projet

- `src/` - Code source principal
  - `app.rs` - Composants d'interface utilisateur et structure de l'application
  - `lib.rs` - Configuration de l'hydratation pour WebAssembly
  - `main.rs` - Point d'entrée du serveur (SSR)
- `style/` - Fichiers SCSS pour les styles
- `public/` - Ressources statiques (images, fonts, etc.)

## Fonctionnalités actuelles

- Structure de base du site web
- Système de routage
- Architecture SSR (Server-Side Rendering) avec hydratation côté client

## Fonctionnalités à venir

- Page d'accueil complète
- Section actualités
- Calendrier des matchs et événements
- Présentation des équipes
- Espace membres
- Galerie photos
- Formulaire de contact

## Prérequis pour le développement

- [Rust](https://www.rust-lang.org/tools/install) (édition 2021 ou plus récente)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) (outil de build pour Leptos)
- [Dart Sass](https://sass-lang.com/install/) (pour compiler les fichiers SCSS)

## Installation et configuration

1. Clonez ce dépôt :
   ```bash
   git clone https://github.com/votre-username/rcc.git
   cd rcc
   ```

2. Installez cargo-leptos (si pas déjà installé) :
   ```bash
   cargo install cargo-leptos
   ```

3. Installez Dart Sass (pour la compilation SCSS)

## Commandes pour lancer le projet

### Développement

Pour lancer le serveur de développement avec rechargement automatique :

```bash
cargo leptos watch
```

Le site sera accessible à l'adresse : http://127.0.0.1:3000

### Construction pour production

Pour construire le projet en mode production :

```bash
cargo leptos build --release
```

Les fichiers compilés seront disponibles dans le dossier `target/site/`.

### Exécution des tests

```bash
cargo test
```

### Tests end-to-end

```bash
cd end2end
npx playwright test
```

## Déploiement

Le site peut être déployé sur n'importe quel serveur capable d'exécuter des binaires Rust et de servir des fichiers statiques.

1. Construisez le projet pour production
2. Transférez le contenu du dossier `target/site/` sur votre serveur
3. Exécutez le binaire généré ou configurez votre serveur web pour servir l'application

## Configuration

La configuration du projet se trouve principalement dans la section `[package.metadata.leptos]` du fichier `Cargo.toml`. Vous pouvez y ajuster différents paramètres comme :

- `site-addr` : L'adresse IP et le port du serveur
- `reload-port` : Le port pour le rechargement automatique
- `style-file` : Le fichier source des styles
- `assets-dir` : Le répertoire des ressources statiques

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à soumettre des pull requests ou à ouvrir des issues pour signaler des bugs ou proposer des améliorations.

## Licence

[Indiquer la licence ici]

---

Créé avec ❤️ pour le Rugby Club Cadaujac.
