# Øresund [ˈøːɐˌsɔnˀ]
Intended to be a small bridge between http and gemini.

It is far from feature complete and can only render the most basic gemini sites
for now. It is also just a poorly built [Rocket](https://rocket.rs/) server
that uses the render part of julienxx's awesome [Asuka](https://git.sr.ht/~julienxx/asuka) browser,
so if you're looking for a more stable gemini browsing experience, go check out
that project.

## Running the project

```bash
$ git clone https://github.com/Briix/oeresund.git
$ cd oeresund
$ rustup override set nightly #rocket needs a nightly version of rust
$ cargo run
```

Now you can visit http://localhost:8000 in your browser to see the project.
From here you can pass any gemini url as the url path to render that site.
**gemini.conman.org** is a good place to start.
```
http://localhost:8000/gemini.conman.org
```

## Known bugs
- It's currently not possible to have the gemini protocol be part of the url
    string. `http://localhost:8000/gemini://gemini.conman.org` will crash the
    server.
