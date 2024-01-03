## Full-Stack Rust ChatBot

We use Leptos in this project, a Rust web framework. Using tailwind for the UI, client and serverside code in Rust, along with a hugging-face LLM that is supported by Rustformers. 

We use Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin. This LLM from hugging-face is supported by Rustformers. As such, this allows us to use the crate models->Llama, along with the methods and functions to first fine-tune the model; outputting a somewhat acceptable chatbot answer. Here is an example snippet: 

![Screenshot from 2024-01-02 16-41-15](https://github.com/sebastian9991/RustyChatBot/assets/61892815/7b10f475-70ac-4d20-9b03-5ee5a73c9df4)



