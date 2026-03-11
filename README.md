
# mdbook-sspaeti

A custom mdBook preprocessor that adds WikiLinks and enhanced image handling.

## Features

### WikiLinks

Converts `[[term]]` to links pointing at a configurable base URL (e.g. a second brain).

| You write | You get |
|-----------|---------|
| `[[Data Governance]]` | `[Data Governance](https://ssp.sh/brain/data-governance)` |
| `[[idempotency\|Idempotence]]` | `[Idempotence](https://ssp.sh/brain/idempotency)` |

### Image WikiLinks (Obsidian-style)

Embed images using Obsidian's `![[filename]]` syntax. Images are resolved from a configurable base path (default: `/images`).

```markdown
<!-- Simple image embed -->
![[roapi.webp]]

<!-- With caption -->
![[roapi.webp|ROAPI architecture overview]]

<!-- Absolute path (base path not prepended) -->
![[/images/subfolder/diagram.svg]]
```

Output for `![[roapi.webp|ROAPI architecture overview]]`:
```html
<figure>
<img src="/images/roapi.webp" alt="ROAPI architecture overview">
<figcaption>ROAPI architecture overview</figcaption>
</figure>
```

### Image Captions (rich markdown)

Place an italic line immediately after an image to create a `<figure>` with `<figcaption>`. The caption supports full inline markdown — links, bold, italic, code.

```markdown
![Data flow](/images/data-flow.webp)
_Image by [Staffbase](https://staffbase.com) about **Digital Workplace** Communication_
```

A blank line between image and caption is also fine:

```markdown
![Data flow](/images/data-flow.webp)

_This caption appears after a blank line and still works_
```

Output:
```html
<figure>
<img src="/images/data-flow.webp" alt="Data flow">
<figcaption>Image by <a href="https://staffbase.com">Staffbase</a> about <strong>Digital Workplace</strong> Communication</figcaption>
</figure>
```

### Image Sizing

Control image width using Pandoc-style attribute syntax:

```markdown
![alt](/images/small-diagram.webp){width=400px}
```

Combine with captions:

```markdown
![alt](/images/small-diagram.webp){width=400px}
_A smaller diagram with a caption_
```

Output:
```html
<figure>
<img src="/images/small-diagram.webp" alt="alt" style="max-width: 400px">
<figcaption>A smaller diagram with a caption</figcaption>
</figure>
```

## Configuration

Add to your `book.toml`:

```toml
[preprocessor.sspaeti]
command = "mdbook-sspaeti"
brain-base-url = "https://ssp.sh/brain"   # base URL for [[wikilinks]]
is-url-check = false                       # validate wikilink URLs on build (slow)
images-base-path = "/images"               # base path for ![[image]] embeds (default: /images)
```

### Options

| Key | Default | Description |
|-----|---------|-------------|
| `brain-base-url` | `""` | Base URL for WikiLink targets |
| `is-url-check` | `false` | HTTP-validate each WikiLink on build |
| `images-base-path` | `/images` | Directory prepended to bare image filenames in `![[...]]` |

## Installation

```bash
cargo install --path .
# or
make build
```

## Recommended CSS

Add figure styling to your mdBook's custom CSS (e.g. `assets/css/customize.css`):

```css
figure {
    margin: 1.5em 0;
    text-align: center;
}
figure img {
    max-width: 100%;
    height: auto;
}
figcaption {
    font-size: 0.9em;
    color: var(--fg);
    opacity: 0.6;
    margin-top: 0.5em;
    font-style: italic;
}
```
