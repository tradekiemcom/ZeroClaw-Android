# Git Guardrail Policy — ZeroClaw-Android

**Inherited from Hub-level policy. MANDATORY for this project.**

## 1. Auto-Commit Before Action

Before any code modification:
1. `git status` — check for uncommitted changes.
2. Clean → `git commit -m "pre-fix: [description]"`
3. Dirty → `git commit -m "wip: save before [task]"`
4. Only then proceed with the task.

## 2. Atomic Commits

One logical change per commit with conventional prefix:
`feat:`, `fix:`, `refactor:`, `docs:`, `test:`, `chore:`, `perf:`, `ci:`, `style:`

## 3. Recovery Protocol

On failure: **STOP** → inform user → offer `git reset --hard HEAD~1` → wait for approval.

## 4. No Force Push

NEVER `git push --force`. Fix mistakes with `git revert`.
