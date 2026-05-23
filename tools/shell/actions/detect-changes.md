# detect-changes.sh

## Purpose

Detect files changed between commits matching a regex pattern

## Usage

```bash
./tools/shell/actions/detect-changes.sh --pattern PATTERN [--compare-with COMPARE_WITH]
```

### GitHub Actions usage:

```yaml
...
on:
  pull_request:
    types: [opened, synchronize, reopened]

jobs:
  detect-changes:
    runs-on: ubuntu-latest
    outputs:
      shell: ${{ steps.shell-changed.outputs.changed }}
      typescript: ${{ steps.typescript-changed.outputs.changed }}

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Shell files changed
        id: shell-changed
        run: |
          bash ./tools/shell/actions/detect-changes.sh \
            --pattern '\.sh$' \
            --compare-with origin/${{ github.base_ref }}
          if [[ -f "$GITHUB_OUTPUT" ]] && grep -q "changed_files" "$GITHUB_OUTPUT"; then
            echo "change=true" >> $GITHUB_OUTPUT
          else
            echo "change=false" >> $GITHUB_OUTPUT
          fi

      - name: TypeScript files changed
        id: typescript-changed
        run: |
          bash ./tools/shell/actions/detect-changes.sh \
            --pattern '\.ts$' \
            --compare-with origin/${{ github.base_ref }}
          if [[ -f "$GITHUB_OUTPUT" ]] && grep -q "changed_files" "$GITHUB_OUTPUT"; then
            echo "change=true" >> $GITHUB_OUTPUT
          else
            echo "change=false" >> $GITHUB_OUTPUT
          fi
...
```

```yaml
...
on:
  push:
    branches: [main]

jobs:
  detect-changes:
    runs-on: ubuntu-latest
    outputs:
      backend: ${{ steps.backend-changed.outputs.changed }}
      frontend: ${{ steps.frontend-changed.outputs.changed }}

    steps:
      - name: Checkout code
        uses: actions/checkout@v4
        with:
          fetch-depth: 0

      - name: Backend changed
        id: backend-changed
        run: |
          bash ./tools/shell/actions/detect-changes.sh \
            --pattern '^src/backend/.*\.(ts)$' \
            --compare-with HEAD~1
          if [[ -f "$GITHUB_OUTPUT" ]] && grep -q "changed_files" "$GITHUB_OUTPUT"; then
            echo "change=true" >> $GITHUB_OUTPUT
          else
            echo "change=false" >> $GITHUB_OUTPUT
          fi

      - name: Drontend changed
        id: frontend-changed
        run: |
          bash ./tools/shell/actions/detect-changes.sh \
            --pattern '^src/frontend/.*\.(ts|html|css)$' \
            --compare-with HEAD~1
          if [[ -f "$GITHUB_OUTPUT" ]] && grep -q "changed_files" "$GITHUB_OUTPUT"; then
            echo "change=true" >> $GITHUB_OUTPUT
          else
            echo "change=false" >> $GITHUB_OUTPUT
          fi
...
```
