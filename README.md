# aegis-orchestrator-proto

Canonical Protocol Buffer definitions for the AEGIS platform. This is the single source of truth for all gRPC contracts shared across AEGIS repositories.

## Versioning

This repo publishes the `aegis-orchestrator-proto` crate and is tagged with semantic versions (for example `0.14.0-pre-alpha`). Those tags drive both GitHub Releases and crates.io publishing. Consumers that use this repo as a git submodule still pin to a tag via their submodule ref.

**Updating the proto:**

1. Make changes to the `.proto` file(s) in this repo
2. Commit and push
3. Tag the new version: `git tag 0.x.y && git push --tags`
4. In each consumer repo, update the submodule ref:

   ```bash
   cd aegis-proto
   git fetch
   git checkout 0.x.y
   cd ..
   git add aegis-proto
   git commit -m "chore: bump aegis-orchestrator-proto to 0.x.y"
   ```

5. Update generated code / TypeScript types in the consumer as needed
6. Submit PRs to all affected consumer repos together

## Consumers

Each consumer adds this repo as a git submodule:

```bash
git submodule add https://github.com/100monkeys-ai/aegis-proto.git aegis-proto
```

### `aegis-orchestrator`

Submodule at `aegis-proto/` (for Rust `tonic-build`) and `embedding-service/aegis-proto/` (for Docker build context).

### `aegis-cortex`

Submodule at `aegis-proto/`. Referenced by `build.rs`.

### `aegis-temporal-worker`

Submodule at `aegis-proto/`. Referenced by `Dockerfile` and loaded at runtime via `@grpc/proto-loader`.

### `aegis-smcp-gateway`

Submodule at `aegis-proto/`. Referenced by `build.rs`.

## Publishing

- GitHub Releases are created from semver tag pushes via [release.yml](.github/workflows/release.yml).
- crates.io publishing runs from the same tag pattern via [crates-publish.yml](.github/workflows/crates-publish.yml).
- The publish workflow expects a `CRATES_TOKEN` GitHub Actions secret.

## License

AGPL-3.0 — see [LICENSE](LICENSE).
