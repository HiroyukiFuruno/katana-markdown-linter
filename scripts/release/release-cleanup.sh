#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="${KATANA_CLEANUP_REPO_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || true)}"
if [[ -z "${REPO_ROOT}" ]]; then
  echo "Could not determine repository root. Run inside a git repository." >&2
  exit 1
fi

if ! cd "$REPO_ROOT"; then
  echo "Could not change to repository root: ${REPO_ROOT}" >&2
  exit 1
fi

RELEASE_TAG="${CLEANUP_RELEASE_TAG:-${CLEANUP_TAG:-}}"
DRY_RUN="${CLEANUP_DRY_RUN:-0}"
SKIP_REMOTE="${CLEANUP_SKIP_REMOTE:-0}"
SKIP_RELEASE_GUARD="${CLEANUP_SKIP_RELEASE_GUARD:-0}"

require_release_published() {
  local tag="$1"
  if [[ "${SKIP_RELEASE_GUARD}" == "1" ]]; then
    return 0
  fi

  if [[ -z "${tag}" ]]; then
    echo "Release tag is required to confirm release publication." >&2
    return 1
  fi

  if ! command -v gh >/dev/null 2>&1; then
    echo "gh CLI is required to verify published release." >&2
    return 1
  fi

  if ! gh release view "${tag}" >/dev/null 2>&1; then
    echo "Release ${tag} not found; cleanup is allowed only after publication." >&2
    return 1
  fi
}

default_branch() {
  local symbol
  if symbol="$(git symbolic-ref refs/remotes/origin/HEAD 2>/dev/null || true)"; then
    echo "${symbol##*/}"
    return
  fi
  echo "master"
}

used_by_worktree() {
  local query="$1"
  local line branch_name
  while IFS= read -r line; do
    if [[ "${line}" == "branch refs/heads/"* ]]; then
      branch_name="${line#branch refs/heads/}"
      if [[ "${branch_name}" == "${query}" ]]; then
        return 0
      fi
    fi
  done < <(git worktree list --porcelain)
  return 1
}

cleanup_local_branches() {
  local base="$1"
  local safe_merge
  safe_merge="$(git branch --merged "${base}" | sed 's/^[ *+]*//')"
  if [[ -z "${safe_merge}" ]]; then
    echo "No local branches merged into ${base}."
    return 0
  fi

  local branch
  local current
  current="$(git branch --show-current)"
  while IFS= read -r branch; do
    [[ -z "${branch}" ]] && continue
    if [[ "${branch}" == "${base}" ]]; then
      continue
    fi
    if [[ "${branch}" == "${current}" ]]; then
      continue
    fi
    if used_by_worktree "${branch}"; then
      echo "skip local branch (in use by worktree): ${branch}"
      continue
    fi

    if [[ "${DRY_RUN}" == "1" ]]; then
      echo "local branch cleanup: ${branch} (dry-run)"
      continue
    fi

    if git branch -d "${branch}" >/dev/null; then
      echo "deleted local branch: ${branch}"
    fi
  done <<<"${safe_merge}"
}

remote_branch_exists() {
  local branch="$1"
  git ls-remote --exit-code --heads origin "${branch}" >/dev/null 2>&1
}

cleanup_remote_branches() {
  if [[ "${SKIP_REMOTE}" == "1" ]]; then
    echo "Skipping remote cleanup (CLEANUP_SKIP_REMOTE=1)"
    return 0
  fi

  local base="$1"
  local merged_remote
  merged_remote="$(git branch -r --merged "origin/${base}" 2>/dev/null | sed 's#^ *origin/##')"
  if [[ -z "${merged_remote}" ]]; then
    echo "No remote branches merged into origin/${base}."
    return 0
  fi

  local branch
  while IFS= read -r branch; do
    [[ -z "${branch}" ]] && continue
    if [[ "${branch}" == "HEAD" || "${branch}" == "${base}" ]]; then
      continue
    fi
    if [[ "${branch}" == *"->"* ]]; then
      continue
    fi
    if ! remote_branch_exists "${branch}"; then
      continue
    fi
    if [[ "${DRY_RUN}" == "1" ]]; then
      echo "remote branch cleanup: ${branch} (dry-run)"
      continue
    fi
    if git push --delete origin "${branch}" >/dev/null; then
      echo "deleted remote branch: ${branch}"
    fi
  done <<<"${merged_remote}"
}

main() {
  local base
  base="$(default_branch)"

  require_release_published "${RELEASE_TAG}"
  git fetch --all --prune
  git switch "${base}"
  git pull --ff-only origin "${base}"

  cleanup_local_branches "${base}"
  cleanup_remote_branches "${base}"
  git worktree prune

  echo "release cleanup completed on ${base}"
}

main "$@"
