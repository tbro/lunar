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
