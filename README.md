## Full-Stack Rust ChatBot

We use Leptos in this project, a Rust web framework. Using tailwind for the UI, client and serverside code in Rust, along with a hugging-face LLM that is supported by Rustformers. 

We use Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin. This LLM from hugging-face is supported by Rustformers. As such, this allows us to use the crate models->Llama, along with the methods and functions to first fine-tune the model; outputting a somewhat acceptable chatbot answer. Here is an example snippet: 

![Screenshot from 2024-01-02 16-41-15](https://github.com/sebastian9991/RustyChatBot/assets/61892815/7b10f475-70ac-4d20-9b03-5ee5a73c9df4)

Once all tensors of the LLM have loaded we can ask an example question, gauging how well it may perform. Here is an example conversation with a rough UI: 

![Screenshot from 2024-01-02 16-46-12](https://github.com/sebastian9991/RustyChatBot/assets/61892815/06535f12-9375-4103-9bc5-90956d9f809f)

As you can see from the first question (What is 6 + 10?) the LLM gives quite a deluded, but impressive-sounding answer, that may stem from the fine-tuning done originally or perhaps some sort of history of questions prior.

The knowledge can sometimes be out of date haha!

![Screenshot from 2024-01-02 17-30-07](https://github.com/sebastian9991/RustyChatBot/assets/61892815/8f1b20a7-778d-4a25-b029-4470720a2e25)


