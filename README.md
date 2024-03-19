
# mdbook-sspaeti
A simple addition to help me publish my book.

For now it convert Wikilinks `[[my link]]` to my Second Brain links `ssp.sh/brain/my-link`.


## Quick How-To

If I have a little time, I might update more, but for now, I can give a quick note about how I set it up:

1. I installed it through `cargo install mdbook-sspaeti` (I guess, maybe this part I did manually)
2. I added this config to my `book.toml`:

```toml
[preprocessor.sspaeti]
command = "mdbook-sspaeti"
brain-base-url = "https://ssp.sh/brain" #link to website you want `[[my-link]]` point to. e.g. `https://ssp.sh/brain/my-link`
is-url-check = true #true ff you want to check if the links are valid. Will take some time

```

The preprocessor will automatically check on each run, if the links are valid and or not like this (might be a bit verbose each time..)
```
...
mdbook-sspaeti- check_link: The URL returned an ERROR: 404 Not Found, http://ssp.sh/brain/streaming-vs-batch-in-orchestration
mdbook-sspaeti- check_link: The URL is valid: http://ssp.sh/brain/semantic-layer
...
```

Hope that helps for now.


<!-- ## Future Features: -->

<!-- ### Admonitions -->
<!-- - Convert Obsidian style admonishions to [mdbook-admonish](https://tommilligan.github.io/mdbook-admonish/) -->
<!-- You write a well structured Rust main.rs that automatically converts admonition styles from Obsidian style to the mdbook-admonish (https://tommilligan.github.io/mdbook-admonish/) needed. -->

<!-- Obsidian style: -->
<!-- ``` -->
<!-- > [!example] --> 
<!-- > My example is the best! -->
<!-- ``` -->

<!-- need to be converted to --> 
<!-- ``` -->
<!-- ```admonish example -->
<!-- My example is the best! -->
<!-- ``` -->

