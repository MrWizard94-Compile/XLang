# M25 local package fixture

`source/` is a complete, locked `local_math@1.0.0` project used by the standard
gate and portable-preview verifier. It declares a tiny exported `double` weave.
The M25 core suite independently creates the equivalent workspace scenario and
proves two consumers resolve one installed package through normal M22
`depends_on` authorization.

The checked-in fixture is source only. Generated bundles, cache entries, and
install outputs belong under an explicit local target directory and are not
checked in:

```powershell
cargo run -p aether-cli -- pkg pack .\examples\package-publish\source\aether.project.json --output .\target\local-math.bundle
cargo run -p aether-cli -- pkg publish .\target\local-math.bundle --cache .\target\local-package-cache
cargo run -p aether-cli -- pkg install --cache .\target\local-package-cache --name local_math --version 1.0.0 --output .\target\local-math
```

See [the M25 contract](../../docs/AETHER_0.37.md) for the exact bundle/cache
format, trust boundary, and explicit non-goals.
