# Release Process

This is the release checklist for Seaography release candidates. Seaography
tracks SeaORM's `2.0.0-rc.N` cadence and is published together with it.

## 1. Choose Version

Check the latest published versions:

```sh
cargo search seaography --limit 1
cargo search sea-orm --limit 1
```

Seaography has its own `2.0.0-rc.N` counter (independent of SeaORM's). Bump to the
next `2.0.0-rc.N`. Note the SeaORM rc you are releasing against (e.g. `2.0.0-rc.40`);
that becomes the dependency floor (see step 3).

## 2. Start Clean

Release from `main` after the change PRs have been merged and CI has passed.

```sh
git switch main
git pull --ff-only
git status --short
```

The worktree should be clean before starting.

## 3. Bump Versions

There is no bump script; update each location by hand. The four publishable crates
are `seaography`, `seaography-macros`, `seaography-cli`, and `seaography-generator`.

1. Crate versions — set `version` in each:
   - `Cargo.toml` (`seaography`)
   - `macros/Cargo.toml` (`seaography-macros`)
   - `generator/Cargo.toml` (`seaography-generator`)
   - `cli/Cargo.toml` (`seaography-cli`)

2. Internal dependency pins — bump to the new version:
   - `Cargo.toml`: `seaography-macros = { version = "~2.0.0-rc.N", ... }`
   - `cli/Cargo.toml`: `seaography-generator = { version = "~2.0.0-rc.N", ... }`

3. SeaORM dependency floor — set to the SeaORM rc you are releasing against. The
   `~2.0.0-rc.N` form floats forward to later rc's and bridges to `2.0.x` stable:
   - `Cargo.toml`: `sea-orm = { version = "~2.0.0-rc.N", ... }`
   - Example crates: `examples/*/Cargo.toml` (`sea-orm` line, plus the
     `# seaography version` pin).

4. Code generator templates — the generated project's `Cargo.toml` hardcodes the
   SeaORM pin; bump it in all three framework templates:
   - `generator/src/templates/poem_cargo.toml`
   - `generator/src/templates/actix_cargo.toml`
   - `generator/src/templates/axum_cargo.toml`

   (The generated `seaography` pin is injected from `CARGO_PKG_VERSION` via
   `generator/src/writer.rs`, so it tracks the bump automatically — no edit needed.)

Sanity check that nothing was missed:

```sh
grep -rn '2.0.0-rc' --include=Cargo.toml . | grep -v '#'
```

## 4. Write Changelog

The 2.0 line uses a single rolling section, `## 2.0.0 - pending`, in `CHANGELOG.md`
— **not** per-rc sections. Add entries (New Features / Enhancements / Bug Fixes)
under it. Leave the `pending` heading as-is; it is only renamed to
`## 2.0.0 - YYYY-MM-DD` at the 2.0.0 final release.

## 5. Validate Locally

```sh
cargo check --workspace --all-features
```

To validate against an unpublished local SeaORM (e.g. the matching rc not yet on
crates.io), temporarily add a patch, check, then revert:

```toml
[patch.crates-io]
sea-orm = { path = "../sea-orm" }
```

Build the examples if backend behavior changed.

## 6. Push and Wait for CI

```sh
git commit -am "2.0.0-rc.N"
git push origin main
```

Wait for GitHub Actions to pass before publishing. Do not publish while CI is red.

## 7. Publish Crates

Publish in dependency order (dependencies before dependents):

1. `seaography-macros`
2. `seaography` (depends on `seaography-macros`)
3. `seaography-generator`
4. `seaography-cli` (depends on `seaography-generator`)

```sh
(cd macros && cargo publish)
cargo publish
(cd generator && cargo publish)
(cd cli && cargo publish)
```

If crates.io indexing causes a dependent publish to fail, wait briefly and retry
the failed crate. The SeaORM rc you depend on must already be published.

## 8. Tag and GitHub Release

```sh
git tag 2.0.0-rc.N
git push origin 2.0.0-rc.N
```

Create a GitHub Release describing the user-visible changes.

## 9. Verify

```sh
cargo search seaography --limit 1
```

Confirm crates.io shows the new versions and that docs.rs builds have started. Spot
check that `seaography-cli` generates a project whose `Cargo.toml` pins the intended
SeaORM and Seaography versions.

## Notes

- While SeaORM is in RC, the dependency **must** keep a pre-release form
  (`~2.0.0-rc.N`); a plain `"2.0"` requirement will not resolve to an rc. Switch to
  a stable requirement only once SeaORM `2.0.0` is published.
- Prefer raising the floor (`~2.0.0-rc.N`) over the bare `~2.0.0-rc` so a published
  Seaography release cannot resolve an older, incompatible SeaORM rc.
