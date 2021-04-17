# lunar cli

just build and run and you get a moon-phase emoji

    cargo run

I made it because I thought it might be fun to have a moon in my [starship prompt](https://starship.rs/). If you would like to do the same you can configure starship to use it.

    [custom.lunar]
    description = "Display moon phase"
    command = "lunar"
    when = 'true'

Of course you will the binary in your `$PATH` for that to work. Cargo makes that easy.

    cargo install --path .

That assumes you have cloned and `cd`ed into this repo. It also assumes you have configured you path to include binaries installed through cargo.

## License

This is just a few lines of code which wraps the [esbat](https://github.com/iliana/esbat) library. In an attempt to avoid trouble it has the same [Creative Commons Attribution-NonCommercial 4.0 International License](https://creativecommons.org/licenses/by-nc/4.0/). Please refer to [that library](https://github.com/iliana/esbat) for details.
