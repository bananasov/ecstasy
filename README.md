# Ecstasy Collector
![Crates.io](https://img.shields.io/crates/v/ecstasy?style=plastic) ![Crates.io](https://img.shields.io/crates/d/ecstasy?style=plastic)

> Check out the original project [here!](https://gitlab.com/lu-ci/kyanite/)

## What Is Ecstasy

[Ecstasy](https://en.wikipedia.org/wiki/Ecstasy_(emotion)) (from Ancient Greek ἔκστασις ékstasis, meaning 'outside of oneself') is a subjective experience of total involvement of the subject, with an object of their awareness. In classical Greek literature it refers to removal of the mind or body "from its normal place of function."

## How to install and run

### Via Cargo

```sh
cargo install ecstasy
# Make sure your $CARGO_HOME/bin is in your $PATH
```

### Installing from source
```sh
git clone https://github.com/Autist69420/ecstasy.git
cd ecstasy
cargo build --release
cargo install --path .
```

> Try `ecstasy --help` for more command line options.

## Supported Services

| Service  | Collector | Location           | Type |
|----------|-----------|--------------------|------|
| e621     | e621      | e621.net           | Tags |
| e926     | e926      | e926.net           | Tags |
| Gelbooru | gelbooru  | gelbooru.com       | Tags |
| Danbooru | danbooru  | danbooru.donmai.us | Tags |
| Konachan | konachan  | konachan.com       | Tags |
| Rule34   | rule34    | rule34.xxx         | Tags |
| Yandere  | yandere   | yande.re           | Tags |
| Xbooru   | xbooru    | xbooru.com         | Tags |
| Realbooru| realbooru | realbooru.com      | Tags |
| Hypnohub | hypnohub  | hypnohub.net       | Tags |
| Safebooru| safebooru | safebooru.org      | Tags |

## Planned Service Support

| Service  | Collector | Location     | Type      |
|----------|-----------|--------------|-----------|
| Pixiv    | pixiv     | pixiv.net    | ?????     |
| nHentai  | nhentai   | nhentai.net  | Album     |
| FAKKU    | fakku     | fakku.net    | Album     |
| E-Hentai | ehentai   | e-hentai.org | Album     |
| Reddit   | reddit    | reddit.com   | Subreddit |
