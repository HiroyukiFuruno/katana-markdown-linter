#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="${KATANA_HOOK_CHECK_REPO_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || true)}"

if [[ -z "${REPO_ROOT}" ]]; then
  echo "Could not determine repository root. Run inside a git repository." >&2
  exit 1
fi

if ! cd "$REPO_ROOT"; then
  echo "Could not change to repository root: ${REPO_ROOT}" >&2
  exit 1
fi

DEFAULT_BRANCH="${KATANA_HOOK_DEFAULT_BRANCH_OVERRIDE:-}"
CURRENT_BRANCH="${KATANA_HOOK_BRANCH_OVERRIDE:-}"
COMMIT_SUBJECT_OVERRIDE="${KATANA_HOOK_COMMIT_SUBJECT_OVERRIDE:-}"
COMMIT_BODY_OVERRIDE="${KATANA_HOOK_COMMIT_BODY_OVERRIDE:-}"
SIMULATED_CHANGED_FILES="${KATANA_HOOK_CHANGED_FILES:-}"

issue_reference() {
  local value=""
  local issue_ref=""

  for value in "$@"; do
    issue_ref="$(printf "%s\n" "$value" | sed -nE 's/.*[Ii]ssue[-_\/ ]*([0-9][0-9]*).*/\1/p')"
    if [[ -n "${issue_ref}" ]]; then
      echo "${issue_ref}"
      return 0
    fi

    issue_ref="$(printf "%s\n" "$value" | sed -nE 's/.*#([0-9][0-9]*).*/\1/p')"
    if [[ -n "${issue_ref}" ]]; then
      echo "${issue_ref}"
      return 0
    fi
  done

  return 1
}

ensure_default_branch() {
  local symbol
  if [[ -n "${DEFAULT_BRANCH}" ]]; then
    echo "${DEFAULT_BRANCH}"
    return
  fi

  if symbol="$(git symbolic-ref "refs/remotes/origin/HEAD" 2>/dev/null || true)"; then
    echo "${symbol##*/}"
    return
  fi

  if git show-ref --verify --quiet refs/remotes/origin/master; then
    echo "master"
    return
  fi

  if git show-ref --verify --quiet refs/remotes/origin/main; then
    echo "main"
    return
  fi

  echo "master"
}

collect_current_branch() {
  if [[ -n "${CURRENT_BRANCH}" ]]; then
    echo "${CURRENT_BRANCH}"
    return
  fi

  git rev-parse --abbrev-ref HEAD
}

collect_commit_message() {
  local subject="${COMMIT_SUBJECT_OVERRIDE}"
  local body="${COMMIT_BODY_OVERRIDE}"
  if [[ -n "${subject}" || -n "${body}" ]]; then
    echo "${subject}"
    echo "${body}"
    return
  fi

  git log -1 --pretty=%B
}

collect_changed_files() {
  local default_branch="$1"
  if [[ -n "${SIMULATED_CHANGED_FILES}" ]]; then
    printf '%s\n' "${SIMULATED_CHANGED_FILES}"
    return
  fi

  if [[ "$#" -gt 1 ]]; then
    shift
    printf '%s\n' "$@"
    return
  fi

  if git show-ref --verify --quiet "refs/remotes/origin/${default_branch}"; then
    git diff --name-only "origin/${default_branch}...HEAD"
    return
  fi

  git diff --name-only HEAD~1
}

has_file_changed() {
  local target="$1"
  shift
  local file
  for file in "$@"; do
    if [[ "${file}" == "${target}" ]]; then
      return 0
    fi
  done
  return 1
}

check_evidence_file() {
  local evidence_file="$1"
  local issue_id="$2"
  if [[ ! -f "${evidence_file}" ]]; then
    echo "Downstream dependency update requires evidence file: ${evidence_file}" >&2
    echo "Create an issue-linked evidence ledger before pushing." >&2
    return 1
  fi

  if ! grep -Eiq '^( |\t)*upstream[[:space:]-]*release[[:space:]]*:' "${evidence_file}"; then
    echo "Evidence file ${evidence_file} missing required marker 'upstream release:'" >&2
    return 1
  fi
  if ! grep -Eiq '^( |\t)*api[[:space:]-]*migration[[:space:]-]*note[[:space:]]*:' "${evidence_file}"; then
    echo "Evidence file ${evidence_file} missing required marker 'api migration note:'" >&2
    return 1
  fi
  if ! grep -Eiq '^( |\t)*verification[[:space:]-]*evidence[[:space:]]*:' "${evidence_file}"; then
    echo "Evidence file ${evidence_file} missing required marker 'verification evidence:'" >&2
    return 1
  fi

  echo "Dependency evidence is present: ${evidence_file}"
  return 0
}

main() {
  local default_branch branch issue_id changed_files manifest_changed=0 lock_changed=0
  local -a files=()

  default_branch="$(ensure_default_branch)"
  branch="$(collect_current_branch)"
  issue_id="$(issue_reference "${branch}" "$(collect_commit_message)")" || true

  while IFS= read -r path; do
    [[ -z "${path}" ]] && continue
    files+=("${path}")
    case "${path}" in
      */Cargo.toml|Cargo.toml|*/pyproject.toml|pyproject.toml|*/package.json|package.json)
        manifest_changed=1
        ;;
      */Cargo.lock|Cargo.lock|*/poetry.lock|poetry.lock|*/package-lock.json|package-lock.json|*/pnpm-lock.yaml|pnpm-lock.yaml|*/yarn.lock|yarn.lock)
        lock_changed=1
        ;;
    esac
  done < <(collect_changed_files "${default_branch}" "$@")

  if [[ "${branch}" == "${default_branch}" || "${branch}" == "HEAD" ]]; then
    echo "Skipping issue-driven gate: default branch ${branch}"
    return 0
  fi

  if [[ -z "${issue_id}" ]]; then
    echo "Issue-driven check failed: non-default branch '${branch}' must include issue reference." >&2
    echo "Use a branch like issue-123 or include #123 in the latest commit message." >&2
    return 1
  fi

  if (( manifest_changed == 1 )); then
    if (( lock_changed == 0 )); then
      echo "Downstream dependency update failed: dependency manifest changed but lockfile did not." >&2
      echo "Update dependency manifests together with lockfile in the same change unit." >&2
      return 1
    fi

    evidence_file="issues/${issue_id}/dependency-update-evidence.md"
    if ! has_file_changed "${evidence_file}" "${files[@]}"; then
      echo "Downstream dependency update failed: missing evidence file in change unit: ${evidence_file}" >&2
      echo "Add ${evidence_file} and include markers in the same change." >&2
      return 1
    fi

    if ! check_evidence_file "${evidence_file}" "${issue_id}"; then
      return 1
    fi
  fi

  echo "issue-driven hook checks passed for branch '${branch}' (issue #${issue_id:-none})"
  return 0
}

main "$@"
