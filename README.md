## IDEA cli

![b](https://img.shields.io/badge/rust-orange?style=for-the-badge&logo=rust)
![b](https://img.shields.io/badge/Open%20AI-grey?style=for-the-badge&logo=openai)
![b](https://img.shields.io/badge/obsidian-purple?style=for-the-badge&logo=obsidian)
![b](https://img.shields.io/badge/ollama-white?style=for-the-badge&logo=ollama)

> **UPDATE** Added Ollama Support!

> CLI tool to generate short ideas into your Obsidian Vault.  
> Read the [blog post](https://medium.com/@rene.krewinkel/that-i-love-the-command-line-is-no-secret-to-the-ones-who-know-me-nor-that-i-build-my-own-511d8ed255ea) 

## Config
Add to your .env file or environment vars

```dotenv
VAULT_PATH=/full/path/to/your/vault
OPENAI_TOKEN=openai-token
OPENAI_URL=https://api.openai.com/v1/
OPENAI_MODEL=gpt-3.5-turbo
USE_OLLAMA=YES|NO
OLLAMA_MODEL=llama2:latest
YOUTUBE_TOKEN=youtube-token-not-used-yet
```

## TO DO
- [x] ~~[Better error handling](https://github.com/ReneKrewinkel/idea-cli/issues/1).~~
- [ ] [Check if there are video's if not, don't generate the content](https://github.com/ReneKrewinkel/idea-cli/issues/2)
- [ ] [Use Official youtube library: google-youtube3](https://github.com/ReneKrewinkel/idea-cli/issues/3)
- [ ] [Integrate in NeoVim (NeoVim plugin!)](https://github.com/ReneKrewinkel/idea-cli/issues/4)
- [ ] [Generic config file in](https://github.com/ReneKrewinkel/idea-cli/issues/5) `${HOME}/.config/idea-cli/settings.json`
- [x] ~~[Option for using Ollama instead of OpenAI](https://github.com/ReneKrewinkel/idea-cli/issues/6)~~
- [ ] [Write tests](https://github.com/ReneKrewinkel/idea-cli/issues/7)
- [x] ~~[fix the `unwrap` in the completion](https://github.com/ReneKrewinkel/idea-cli/issues/8)~~
- [ ] [Strange behaviour in the return of the completion. somehow the string isn't trimmed properly returns a `\\u0000-\\u00fn`](https://github.com/ReneKrewinkel/idea-cli/issues/9)
- [ ] **Idea:** Spawn a thread to run it in the background?
- [ ] ***Optimize code!!!!!***

## Build
#### MacOS
```shell
cargo build --target=aarch64-apple-darwin --release
```

#### Windows
```shell
cargo build --target x86_64-pc-windows-gnu
```