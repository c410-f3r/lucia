# Adding a new API

Simply put, the only necessary thing to create an endpoint is implementing the `Request` trait. However, this project expects more from an API definition.

## Location

Take a look at https://github.com/public-apis/public-apis to see which category your API fits better. If the `src/api` directory doesn't contain such category, then create one and also put it in the `README.md` file as well as the API name in alphabetical order.

```bash
cd src/api/game
touch my_new_gaming_api.rs;
```

Name | URL  | Pct |
|---|---|---|
| First API | Some URL | 100% |
| ... | ... | ... |
| **My new gaming API** | **My new gaming API URL** | **0%** |
| ... | ... | ... |
| Last API | Some URL | 100% |

## Development

### Feature flag

To avoid bringing unused code or unnecessary dependencies that otherwise would slow down compilation, all APIs are placed behind a build flag.

```toml
# Cargo.toml

[features]
first-api = []
...
my-new-gaming-api = [] # Add any required dependency
...
last-api = []
```

```rust
// src/api/game.rs

#[cfg(feature = "first-api")]
pub mod first_api;
...
#[cfg(feature = "my-new-gaming-api")]
pub mod my_new_gaming_api;
...
#[cfg(feature = "last-api")]
pub mod last_api;
```

### Endpoints

This is a redundant and laborious step that is currently handled by declarative macros that will be removed in favour of procedural macros. As such, more documentation about the subject will be delivered as soon as procedural macros are done.

## Tests

Integration tests or end-to-end testing can be performed using an internet connection or through the local `Test` structure.

APIs are numerous and generally built based on third-parties specifications so it makes sense to perform external calls.

```bash
cd src/api/game/my_new_gaming_api
touch integration_tests.rs;
```

Don't forget to also add local and CI coverage.

```bash
# .scripts/internet-tests.sh

$rt check-with-features . first-api
...
$rt check-with-features . my-new-gaming-api
...
$rt check-with-features . last-api
```

.scripts/internal-tests
Cargo.toml
integration-tests


# Calling tests

All tests of all types can be issued using normal `cargo` commands but more complex and automatic coverage is currently dealt with Bash scripts. Multi-platform scripts is something that can be resolved in the future with enough interest.

Continuous Integration uses what is inside the `.script` directory and so you can in a local environment.

* **.scripts/integration-tests.sh**: Tests if the internal declarations conform the external counterparts. May or may not require a internet connection.
* **.scripts/internal-tests.sh**: Unit tests, formatting and lints.
* **.scripts/spin-up-local-instances.sh**: Used by `integration-tests.sh` to test write-operations that would otherwise incur real expenses.

```bash
# Internal tests

cd lucia;
.scripts/internal-tests.sh
```

```bash
# Integration tests

# Terminal 1
cd lucia;
.scripts/spin-up-local-instances.sh

# Terminal 2
cd lucia;
.scripts/integration-tests.sh
```
