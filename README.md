# No-emoji

I hate how LLMs insist on putting non-ascii characters in comments and strings.
This is a super simple pre-commit hook  that rejects changes containing non-ascii characters.

Ideally the specifics of the valid/invalid character sets would be configurable, but for now it's
just based on whatever the rust [std::Char::is_ascii()](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii)
function does.


## Example .pre-commit-config.yaml
```yaml
repos:
  - repo: https://github.com/B1tWhys/no-emoji.git
    rev: 1.0
    hooks:
      - id: no-emoji
        exclude: ".*\\.md" # exclude markdown files
```
