# Review Policy

This policy governs how code reviews are conducted in this environment.
It applies uniformly regardless of which coding agent runtime is executing the review.

## Scope

This policy covers:
- Branch delta reviews (pull request reviews)
- Patch reviews
- Architecture change reviews

## Review Objectives

A review must evaluate code changes across four dimensions:

1. **Correctness** — Does the code do what it claims? Are there bugs?
2. **Architecture** — Does the code follow the project's architecture rules?
3. **Risk** — Does the change introduce risk (security, data loss, performance)?
4. **Readability** — Will the next developer understand this code?

## Severity Levels

| Level    | Meaning                                                   | Action Required         |
|----------|-----------------------------------------------------------|-------------------------|
| CRITICAL | Bug or vulnerability that will cause production failure   | Must be fixed before merge |
| HIGH     | Significant risk or clear architecture violation          | Must be addressed       |
| MEDIUM   | Code smell, minor architecture drift, or logic concern    | Should be addressed     |
| LOW      | Style, naming, or readability improvement                 | Nice to fix             |
| INFO     | Observation with no required action                       | Informational only      |

## Required Checks

Every review must check for:

### Correctness
- [ ] Edge cases handled (null, empty, overflow, negative)
- [ ] Error paths handled and propagated correctly
- [ ] No off-by-one errors in loops and ranges
- [ ] Async/concurrent code is safe

### Security
- [ ] No secrets committed to source control
- [ ] User input is validated and sanitized
- [ ] No SQL injection, XSS, or command injection vectors
- [ ] Authentication and authorization are not bypassed

### Architecture
- [ ] Changes follow the rules in `shared/architecture-rules.md`
- [ ] No circular dependencies introduced
- [ ] Abstraction layers are respected
- [ ] No direct coupling between layers that should be isolated

### Testing
- [ ] New behavior has test coverage
- [ ] Existing tests still pass (or are updated with justification)
- [ ] Tests test behavior, not implementation details

## Output Format

Reviews must be structured as follows:

```
## Summary
[One paragraph: what does this branch do, what changed, overall impression]

## Bug Findings
- [SEVERITY] file.ext:LINE — description
  Suggestion: how to fix

## Architecture Concerns
- [SEVERITY] description
  Reference: architecture-rules.md#section

## Risky Changes
- [SEVERITY] description
  Risk: what could go wrong
  Mitigation: suggested action

## Recommendation
[APPROVE | REQUEST_CHANGES | NEEDS_DISCUSSION]
Rationale: [one sentence]
```

## Approval Criteria

- **APPROVE**: No CRITICAL or HIGH findings. All MEDIUM findings have been acknowledged.
- **REQUEST_CHANGES**: One or more CRITICAL or HIGH findings must be resolved.
- **NEEDS_DISCUSSION**: Architecture or design questions require team input before proceeding.

## What Reviews Must NOT Do

- Nitpick style issues without offering a concrete improvement
- Block on personal preference without a documented rule
- Approve code with known CRITICAL issues
- Leave findings without severity labels
