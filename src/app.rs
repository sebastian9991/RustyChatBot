use crate::api::converse;
use crate::app::components::chat_area::ChatArea;
use crate::app::components::type_area::TypeArea;
use crate::model::conversation::Conversation;
use crate::model::conversation::Message;
use leptos::*;
use leptos_meta::*;
mod components;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (conversation, set_conversations) = create_signal(Conversation::new());

    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversations.update(move |c| {
            c.messages.push(user_message);
        });
        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversations.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversations.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Hello there, General Kenobi."/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}
