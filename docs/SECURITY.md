# Security Guide

This document outlines security considerations when using `pin-actions` and GitHub Actions in general.

## Why Pin Actions?

### The Problem

GitHub Actions can be referenced using:
- **Tags** (e.g., `@v4`) - Can be moved to point to different commits
- **Branches** (e.g., `@main`) - Constantly changing
- **SHAs** (e.g., `@abc123...`) - Immutable, cannot be changed

### Attack Scenarios

1. **Tag Manipulation**
   ```yaml
   # Attacker compromises actions/checkout repository
   # Moves v4 tag to malicious commit
   - uses: actions/checkout@v4  # ⚠️ Now runs malicious code
   ```

2. **Branch Manipulation**
   ```yaml
   # Attacker pushes malicious code to main branch
   - uses: actions/setup-node@main  # ⚠️ Runs latest malicious code
   ```

3. **Supply Chain Attack**
   - Compromised maintainer account
   - Repository takeover
   - Dependency confusion

### The Solution

Pin to specific commit SHAs:

```yaml
# Immutable reference - cannot be changed
- uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
```

## Security Best Practices

### 1. Verify SHAs Before Pinning

```bash
# Always verify the SHA matches the expected version
git ls-remote https://github.com/actions/checkout.git v4

# Cross-reference with release page
# https://github.com/actions/checkout/releases/tag/v4
```

### 2. Keep Pins Updated

Pinning doesn't mean "set and forget":

```yaml
# Setup automated updates
name: Update Pins
on:
  schedule:
    - cron: '0 0 1 * *'  # Monthly

jobs:
  update:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: |
          pin-actions
          # Review changes
          # Run tests
          # Create PR
```

### 3. Use Dependabot or Renovate

Automate SHA updates:

```yaml
# .github/dependabot.yml
version: 2
updates:
  - package-ecosystem: "github-actions"
    directory: "/"
    schedule:
      interval: "weekly"
    labels:
      - "dependencies"
      - "security"
```

### 4. Audit Third-Party Actions

Before using any action:

1. **Check repository ownership**
   - Is it from a trusted organization?
   - Is it well-maintained?

2. **Review the code**
   ```bash
   git clone https://github.com/actions/checkout
   cd checkout
   git checkout v4
   # Review the code
   ```

3. **Check security advisories**
   - Look for CVEs
   - Check GitHub security tab

4. **Verify release signatures**
   ```bash
   # If available
   gh release view v4 --repo actions/checkout
   ```

### 5. Principle of Least Privilege

```yaml
# Limit permissions
permissions:
  contents: read
  
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@abc123...
```

### 6. Use Private Actions When Possible

```yaml
# For sensitive operations, use private actions
- uses: ./.github/actions/deploy
  # Under your control, no supply chain risk
```

## Threat Model

### What Pinning Protects Against

✅ **Tag/Branch Manipulation**
- Tags moved to malicious commits
- Branches updated with malicious code

✅ **Compromised Repositories**
- After you pin, attacker cannot inject code

✅ **Typosquatting**
- Pinning forces explicit verification

### What Pinning Does NOT Protect Against

❌ **Initial Compromise**
- If the SHA you pin is already malicious

❌ **Local Repository Attacks**
- Attacker with write access to your repo

❌ **GitHub Platform Compromise**
- If GitHub itself is compromised

❌ **Vulnerabilities in Pinned Code**
- Old vulnerabilities in pinned versions

## Security Checklist

### Before Pinning

- [ ] Verify you're pinning the correct repository
- [ ] Check the action's source code
- [ ] Review release notes and changelog
- [ ] Verify SHA matches the expected version
- [ ] Check for known vulnerabilities

### After Pinning

- [ ] Set up automated SHA updates
- [ ] Monitor for security advisories
- [ ] Review changes in update PRs
- [ ] Test thoroughly after updates
- [ ] Maintain audit trail of changes

### Regular Maintenance

- [ ] Monthly: Review pinned versions
- [ ] Quarterly: Audit all actions
- [ ] Annually: Security review of workflows
- [ ] Always: Monitor security advisories

## Secure Workflow Examples

### Minimal Permissions

```yaml
name: Secure CI
on: [push]

permissions:
  contents: read
  
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
        with:
          persist-credentials: false
      
      - uses: actions/setup-node@1e60f620b9541d16bece96c5465dc8ee9832be0b # v3
        with:
          node-version: '18'
      
      - run: npm ci
      - run: npm test
```

### Secure Secrets Handling

```yaml
name: Deploy
on:
  push:
    branches: [main]

permissions:
  contents: read
  id-token: write  # For OIDC

jobs:
  deploy:
    runs-on: ubuntu-latest
    environment: production
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
      
      # Use OIDC instead of long-lived credentials
      - uses: aws-actions/configure-aws-credentials@v4
        with:
          role-to-assume: arn:aws:iam::123456789012:role/GitHubActions
          aws-region: us-east-1
      
      - run: aws s3 sync ./dist s3://my-bucket/
```

### Input Validation

```yaml
name: Build
on:
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to build'
        required: true

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4
      
      # Validate input
      - name: Validate version
        run: |
          if ! [[ "${{ inputs.version }}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            echo "Invalid version format"
            exit 1
          fi
      
      - name: Build
        run: ./build.sh "${{ inputs.version }}"
```

## Incident Response

### If You Suspect Compromise

1. **Immediately disable the workflow**
   ```bash
   gh workflow disable workflow-name
   ```

2. **Investigate**
   - Review recent commits
   - Check workflow runs
   - Audit action versions

3. **Remediate**
   - Update to clean versions
   - Rotate secrets
   - Review access logs

4. **Document**
   - Timeline of events
   - Impact assessment
   - Lessons learned

### If an Action is Compromised

1. **Check if you're affected**
   ```bash
   grep -r "compromised/action" .github/workflows/
   ```

2. **Update immediately**
   ```bash
   # Pin to last known good version
   pin-actions --workflows-dir .github/workflows
   ```

3. **Audit workflow runs**
   - Review logs for suspicious activity
   - Check for data exfiltration

4. **Rotate secrets**
   ```bash
   # All secrets that might be exposed
   gh secret set SECRET_NAME --body "new-value"
   ```

## Compliance and Auditing

### Audit Trail

Track all pin updates:

```bash
# Git log shows who changed what
git log --follow .github/workflows/ci.yml

# Include rationale in commits
git commit -m "chore: update actions/checkout to v4.2.0 (security fix for CVE-2024-XXXX)"
```

### Compliance Requirements

For compliance frameworks (SOC2, ISO 27001, etc.):

1. **Document your pinning policy**
2. **Regular security reviews**
3. **Automated updates with approval**
4. **Incident response plan**
5. **Audit trail maintenance**

### SBOM Generation

Generate Software Bill of Materials:

```bash
# List all actions used
grep -r "uses:" .github/workflows/ | \
  sed 's/.*uses: //' | \
  sort -u > actions-sbom.txt
```

## Resources

### Official Documentation

- [GitHub Actions Security](https://docs.github.com/en/actions/security-guides)
- [SLSA Framework](https://slsa.dev/)
- [OpenSSF Scorecard](https://github.com/ossf/scorecard)

### Security Tools

- [actionlint](https://github.com/rhysd/actionlint) - Workflow linter
- [Scorecard Action](https://github.com/ossf/scorecard-action) - Security scoring
- [StepSecurity](https://github.com/step-security) - Security hardening

### Community

- [GitHub Actions Security Best Practices](https://github.com/actions/toolkit/blob/main/docs/security.md)
- [Action Security Checklist](https://github.com/actions/toolkit/blob/main/docs/action-security-checklist.md)

## Reporting Security Issues

If you discover a security issue in pin-actions:

**DO NOT** open a public issue.

Instead, email security@example.com with:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will respond within 48 hours.
