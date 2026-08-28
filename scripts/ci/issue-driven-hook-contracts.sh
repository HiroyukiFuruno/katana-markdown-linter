#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "${REPO_ROOT}" ]]; then
  echo "Could not determine repository root." >&2
  exit 1
fi

HOOK_SCRIPT="${REPO_ROOT}/scripts/hooks/issue-driven-git-hook.sh"
if [[ ! -x "${HOOK_SCRIPT}" ]]; then
  echo "Issue-driven hook script missing: ${HOOK_SCRIPT}" >&2
  exit 1
fi

WORK_ROOT="$(mktemp -d)"
WORK_REPO="${WORK_ROOT}/repo"
git -C "${REPO_ROOT}" worktree add --detach "${WORK_REPO}" HEAD

DEFAULT_BRANCH="$(git -C "${WORK_REPO}" symbolic-ref refs/remotes/origin/HEAD 2>/dev/null | sed 's#refs/remotes/origin/##' || true)"
if [[ -z "${DEFAULT_BRANCH}" ]]; then
  DEFAULT_BRANCH="master"
fi

cleanup() {
  if [[ -d "${WORK_ROOT}" ]]; then
    git -C "${REPO_ROOT}" worktree remove --force "${WORK_REPO}" >/dev/null 2>&1 || true
  fi
}
trap cleanup EXIT

run_case() {
  local name expected_status branch subject files evidence_file
  name="$1"
  expected_status="$2"
  branch="$3"
  subject="$4"
  files="$5"
  evidence_file="$6"
  local output status

  if [[ -n "${evidence_file}" ]]; then
    rm -rf "${WORK_REPO}/issues"
    mkdir -p "$(dirname "${WORK_REPO}/${evidence_file}")"
    cat >"${WORK_REPO}/${evidence_file}" <<'EOF'
upstream release: dependency update baseline
api migration note: this is a contract test fixture
verification evidence: local fixture run
EOF
  else
    rm -rf "${WORK_REPO}/issues"
  fi

  set +e
  output="$(KATANA_HOOK_CHECK_REPO_ROOT="${WORK_REPO}" \
    KATANA_HOOK_DEFAULT_BRANCH_OVERRIDE="${DEFAULT_BRANCH}" \
    KATANA_HOOK_BRANCH_OVERRIDE="${branch}" \
    KATANA_HOOK_COMMIT_SUBJECT_OVERRIDE="${subject}" \
    KATANA_HOOK_CHANGED_FILES="${files}" \
    bash "${HOOK_SCRIPT}" 2>&1)"
  status=$?
  set -e

  if [[ "${status}" -ne "${expected_status}" ]]; then
    printf 'Case %q failed. expected exit %s, got %s\n' "${name}" "${expected_status}" "${status}" >&2
    printf '%s\n' "${output}" >&2
    return 1
  fi

  echo "PASS: ${name}"
}

run_case "default branch skip" 0 "${DEFAULT_BRANCH}" "chore: update docs" "README.md" ""
run_case "non-issue branch rejected" 1 "issue-task" "feat: update" "README.md" ""
run_case "manifest change without lockfile fails" 1 "issue-123" "feat: dependency update" "Cargo.toml" ""
run_case "manifest/lock without evidence fails" 1 "issue-123" "feat: dependency update" $'Cargo.toml\nCargo.lock' ""
run_case "manifest/lock with evidence passes" 0 "issue-123" "feat: dependency update" $'Cargo.toml\nCargo.lock\nissues/123/dependency-update-evidence.md' "issues/123/dependency-update-evidence.md"

echo "issue-driven hook contract tests completed"
