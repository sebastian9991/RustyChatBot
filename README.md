## Full-Stack Rust ChatBot
![ezgif-2-3b8a68f378](https://github.com/sebastian9991/RustyChatBot/assets/61892815/a73046d9-d0cd-4853-bf0c-76f1180caec0)


We use Leptos in this project, a Rust web framework. Using tailwind for the UI, client and serverside code in Rust, along with a hugging-face LLM that is supported by Rustformers. 

We use Wizard-Vicuna-7B-Uncensored.ggmlv3.q8_0.bin. Rustformers rely on the use of GGML which uses a technique called "quantization" allowing for almost a compressed version of LLMs to run on local hardware. In a simple explanation, Quantization just reduces the precision of weights in the transformer model, reducing the resources required to store and use the model. As you can imagine this has some tradeoffs in efficiency and performance. 

This LLM from hugging-face is supported by Rustformers. As such, this allows us to use the crate models->Llama, along with the methods and functions to first fine-tune the model; outputting a somewhat acceptable chatbot answer. Here is an example snippet: 

![Screenshot from 2024-01-02 16-41-15](https://github.com/sebastian9991/RustyChatBot/assets/61892815/7b10f475-70ac-4d20-9b03-5ee5a73c9df4)

Once all tensors of the LLM have loaded we can ask an example question, gauging how well it may perform. Here is an example conversation with a rough UI: 

![Screenshot from 2024-01-02 16-46-12](https://github.com/sebastian9991/RustyChatBot/assets/61892815/06535f12-9375-4103-9bc5-90956d9f809f)

As you can see from the first question (What is 6 + 10?) the LLM gives quite a deluded, but impressive-sounding answer, that may stem from the fine-tuning done originally or perhaps some sort of history of questions prior.

The knowledge can sometimes be out of date haha!

![Screenshot from 2024-01-02 17-30-07](https://github.com/sebastian9991/RustyChatBot/assets/61892815/8f1b20a7-778d-4a25-b029-4470720a2e25)


