# Jelly 🍓

> **Note**: Jelly is being built according to [README-driven development][rdd], whereby the README comes first and development follows.
> Although I'm actively working on it, very little of what's described in the README has come to fruition.

Jelly is a next-generation tool for building technical documentation sites for software projects.
What makes it next gen?

- **Lean**.
  No JavaScript, no `package.json`, no `node_modules`.
  Just a single static binary written in [Rust] that does everything.
- Built-in **content management**.
  Point Jelly at nested directories of properly formed [Markdown] (and some other supported inputs) and it builds site navigation, taxonomies, and more.
- Built-in **best practices**.
  Prose linting à la [Vale], link checking, SEO metadata, and more.
- Robust **search**.
  Use local site search or generate JSON search indices for popular platforms like [Algolia] and [Elasticsearch][es].
- **Scalable**.
  Use Jelly to build a single project or federate many projects together under a single [realm](#realms).
- **Extensible**.
  Though Jelly strives to cover most use cases out of the box, it also provides a powerful [extension mechanism](#extensions) to cover the remaining ground.
- **Beautiful**.
  Docs should never be ugly.
  Jelly comes with a variety of lovely built-in components, configurable themes, and dark mode.

Jelly offers you **fewer choices**, **more guardrails**, and **just enough configurability** to cover a huge chunk of use cases with little fuss.
It's the result of a decade of experience&mdash;and a not-inconsiderate amount of toil&mdash;documenting software projects small and large.

## Installation

You can install Jelly on macOS or Linux using [Homebrew]:

```shell
brew install jelly
```

## Create a new project

To fire up a starter project to experiment with:

```shell
jelly init my-docs-project && cd my-docs-project
```

What you'll find in the project:

- A `jelly.yaml` config file in the root
- A `docs` directory with some sample [Markdown] docs nested several directories deep
- An `api` directory with an [OpenAPI] specification

And that's it!
Nothing else is needed to build your site.

## Run the project locally

To run the site in dev mode:

```shell
jelly dev
```

Open your browser to http://localhost:3000 to see your running site!
Navigate to http://localhost:3000/docs to see the landing page for the documentation or to http://localhost:3000/api to see rendered [OpenAPI] docs.

## Build the project

```shell
jelly build
```

This generates the full static site in the `dist` directory.
Because the site is static, Jelly-built sites can be published on just about any platform.

## Configuration

All configuration for Jelly is handled in the `jelly.yaml` file in the project root.
The table below shows the available parameters.

| Parameter          | Meaning                                                     | Required | Default   |
| :----------------- | :---------------------------------------------------------- | :------- | :-------- |
| `title`            | The site title                                              | ✅       |           |
| `description`      | A brief description of the site                             | ❌       |           |
| `repo`             | The URL of the repository for the project                   | ❌       |           |
| `colors.primary`   | The primary color for the theme                             | ❌       | `#123456` |
| `colors.secondary` | The secondary color for the theme                           | ❌       | `#654321` |
| `search`           | Search setup. Options are `local`, `algolia`, and `elastic` | ❌       | `local`   |

## Markdown components

Jelly provides numerous Markdown components out of the box:

- Syntax highlighting.

You can also create [custom components](#extensions).

## Concepts

Jelly aims to be straightforward but there are two concepts worth familiarizing yourself with.

### Extensions

Jelly enables you to extend its core functionality in several ways:

1. Overwrite existing templates. Provide your own `templates` directory and any `.html` file in it overwrites one of the built-ins (such as `page.html`, `toc.html`, or `nav.html`).
2. Provide extension **bundles**.
   Bundles are collections of files&mdash;JavaScript, CSS, etc.&mdash;inside a directory that can be inserted into Jelly sites.
   A bundle is any directory with a `jelly-bundle.yaml` file at the root that specifies what to include in it.
   You can use directories on your machine as bundles or target remote [Git] repositories.
3. Custom Markdown components.
   Jelly uses [MDX] syntax for custom components (though it doesn't actually implement MDX).
   Jelly passes arguments from those components to [TypeScript] functions that you can include in an extension bundle.

The Jelly CLI makes it easy to create and develop extensions:

```shell
jelly extension create my-extension && cd my-extension
jelly extension test
```

#### Built-in extensions

Jelly currently ships with a few built-ins:

| Plugin    | What it does                                                                        |
| :-------- | :---------------------------------------------------------------------------------- |
| `mermaid` | Use [Mermaid] diagrams inside your [Markdown] content                               |
| `openapi` | Transforms [OpenAPI] specifications in [YAML] into beautiful rendered documentation |
| `cli`     | Generate docs for command-line tools from structured YAML sources                   |

### Realms

Sometimes you need to expand beyond a single docs project and create many projects tied together.
For that, Jelly offers **realms**, which group an indefinite number of projects together.
Realms are coordinated by a central server that keeps track of which projects exist, handles things like global search, and more.

You can create a realm by running a Jelly server:

```shell
jelly serve-realm
```

Jelly sites can join the realm by adding a `realm` parameter in `jelly.yaml` specifying the address of the server.

```yaml
realm: https://my-jelly-server.com
```

If you specify a realm, you can publish your site whenever you make changes:

```shell
jelly publish
```

[algolia]: https://algolia.com
[es]: https://github.com/elastic/elasticsearch
[git]: https://git-scm.com
[homebrew]: https://brew.sh
[markdown]: https://markdownguide.org
[mermaid]: https://mermaid-js.github.io
[mdx]: https://mdxjs.com
[openapi]: https://openapis.org
[rdd]: https://tom.preston-werner.com/2010/08/23/readme-driven-development
[rust]: https://rust-lang.org
[typescript]: https://typescriptlang.org
[vale]: https://vale.sh
[yaml]: https://yaml.org
