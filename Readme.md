<h2 align="center">Manner-rs</h2>

"Where are my manners?" <br>
This projects is meant to give you a random command summary (short man page) each time you open a terminal session. It should be a fun way to learn and check new commands you never knew about in a matter of seconds.

### Installation and usage

To proper install manners I suggest you clone this repo and then run the following commands:

```bash
chmod +x scripts/install.sh

# use --bash instead if you have bash as default
./scripts/install.sh --zsh
```
If you install it like this, an allias will be created and the way to call it on demand would be by writing "manners" like below. Please note that only by using the installation script you will get a command when you fire a new terminal session.
```bash
manners
```


Alternatively you could install it with cargo like this

```bash
cargo install --git https://github.com/vlad-onis/manner-rs
```

And then run it by calling it's name in the terminal
```bash
manner-rs
```