#[cfg(test)]
mod tests {
    use chat_bot_rust::model::conversation::Conversation;
    use chat_bot_rust::model::conversation::Message;
    #[test]
    fn test_component_iterable() {
        let mut prompt = Conversation::new();
        let m1 = Message {
            user: true,
            text: "Hello this is a test m1".to_string(),
        };
        let m2 = Message {
            user: true,
            text: "Hello this is a test m1".to_string(),
        };
        let m3 = Message {
            user: true,
            text: "Hello this is a test m1".to_string(),
        };
        prompt.messages.push(m1);
        prompt.messages.push(m2);
        prompt.messages.push(m3);

        for message in prompt.messages {
            println!("{:?}", message);
        }
    }
}
