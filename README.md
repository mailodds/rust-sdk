# MailOdds SDK for Rust

[![crates.io](https://img.shields.io/crates/v/mailodds)](https://crates.io/crates/mailodds)

Official Rust client for the [MailOdds Email Validation API](https://mailodds.com/docs).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mailodds = "1.0"
```

## Quick Start

```rust
use mailodds::apis::configuration::Configuration;
use mailodds::apis::email_validation_api::validate_email;
use mailodds::models::{ValidateRequest, validation_response::Action};

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("mo_live_your_api_key".to_string());

    let request = ValidateRequest::new("user@example.com".to_string());
    let result = validate_email(&config, request).await.unwrap();

    match result.action {
        Action::Accept => println!("Safe to send"),
        Action::AcceptWithCaution => println!("Valid but risky -- flag for review"),
        Action::Reject => println!("Do not send"),
        Action::RetryLater => println!("Temporary failure -- retry after backoff"),
    }
}
```

## Response Handling

Branch on the `action` field for decisioning:

| Action | Meaning | Recommended |
|--------|---------|-------------|
| `accept` | Safe to send | Add to mailing list |
| `accept_with_caution` | Valid but risky (catch-all, role account) | Flag for review |
| `reject` | Invalid or disposable | Do not send |
| `retry_later` | Temporary failure | Retry after backoff |

## Test Mode

Use an `mo_test_` prefixed API key with test domains for predictable responses without consuming credits.

## API Reference

Full API documentation: https://mailodds.com/docs
OpenAPI spec: https://mailodds.com/openapi.yaml

## License

MIT
