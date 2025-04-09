use leptos::prelude::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div class="contact-page">
            <h1>"Contact"</h1>
            <p>"Contactez-nous pour plus d'informations."</p>
            <form class="contact-form">
                <div class="form-group">
                    <label for="name">"Nom"</label>
                    <input type="text" id="name" name="name" required/>
                </div>
                <div class="form-group">
                    <label for="email">"Email"</label>
                    <input type="email" id="email" name="email" required/>
                </div>
                <div class="form-group">
                    <label for="message">"Message"</label>
                    <textarea id="message" name="message" rows="5" required></textarea>
                </div>
                <button type="submit" class="submit-button">"Envoyer"</button>
            </form>
        </div>
    }
}
