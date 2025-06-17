# mauve-connect-check

A command-line tool for verifying DNS configuration of domains in the Mauve Cloud infrastructure.

## Overview

`mauve-connect-check` queries the Mauve Cloud domain manager API to verify whether a domain's DNS records match the expected configuration. It provides detailed, color-coded output showing which records pass, fail, or need attention.

## Features

- **DNS Record Verification**: Checks A, AAAA, and CNAME records against expected values
- **Colored Output**: Clear visual feedback with color coding:
  - ✅ Green: Passed checks
  - ❌ Red: Failed checks
  - ⚠️ Yellow: Warnings (e.g., legacy configurations)
- **Wildcard Support**: Handles wildcard DNS entries (e.g., `*.example.com`)
- **Legacy Detection**: Warns about deprecated configurations and suggests improvements
- **Detailed Diffs**: Shows expected vs actual DNS values for easy troubleshooting

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/mauve-connect-check.git
cd mauve-connect-check

# Build and install
cargo install --path .
```

### Prerequisites

- Rust 1.70 or later
- Internet connection (for API queries)

## Usage

```bash
mauve-connect-check <domain>
```

### Examples

```bash
# Check a single domain
mauve-connect-check example.com

# Check a subdomain
mauve-connect-check api.example.com

# Check a wildcard domain
mauve-connect-check "*.example.com"
```

## Exit Codes

The tool uses different exit codes to indicate the result:

- `0`: All checks passed
- `-1`: One or more checks failed
- `1`: Missing required argument
- `2`: Error during check (network, API, etc.)

## Output Format

The tool provides a structured output showing:

1. Domain being checked
2. Overall check status
3. Detailed record-by-record results including:
   - Record type (A, AAAA, CNAME)
   - Expected value
   - Actual value
   - Status (OK, NOT OK, etc.)

Example output:
```
Domain: mauve.de
Status: success

A mauve.de
Status: passed
incomplete
Values:
  10.10.0.1: ok
  10.10.0.2: not found

CNAME www.mauve.de
Status: passed
Values:
  frontend.mauve.: ok

```
```

## Development

### Building

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Dependencies

- `anyhow`: Error handling
- `colored`: Terminal output colorization
- `reqwest`: HTTP client for API calls
- `serde` & `serde_json`: JSON serialization
- `tokio`: Async runtime

## License

(c) Mauve Mailorder Software GmbH & Co. KG, 2025. Licensed under MIT license.

## Author

[Daniel Brendgen-Czerwonk](https://github.com/czerwonk)
