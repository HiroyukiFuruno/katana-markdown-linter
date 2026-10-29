from __future__ import annotations

import json
import pathlib
import re
from dataclasses import dataclass


@dataclass(frozen=True)
class Sample:
    id: str
    execution_stem: str
    kind: str
    input_path: pathlib.Path
    answer_path: pathlib.Path
    source_repository: str
    source_commit: str
    source_path: str
    license: str
    retrieved_at: str
    selection_reason: str
    historical_patterns: list[str]
    answer_reviewed: bool
    answer_review_note: str

    @classmethod
    def from_payload(
        cls,
        payload: dict[str, object],
        root: pathlib.Path,
        ordinal: int,
    ) -> "Sample":
        return cls(
            id=cls.string(payload, "id"),
            execution_stem=f"sample-{ordinal:06d}",
            kind=cls.string(payload, "kind"),
            input_path=cls.path(payload, root, "input_path"),
            answer_path=cls.path(payload, root, "answer_path"),
            source_repository=cls.string(payload, "source_repository"),
            source_commit=cls.string(payload, "source_commit"),
            source_path=cls.string(payload, "source_path"),
            license=cls.string(payload, "license"),
            retrieved_at=cls.string(payload, "retrieved_at"),
            selection_reason=cls.string(payload, "selection_reason"),
            historical_patterns=cls.string_list(payload, "historical_patterns"),
            answer_reviewed=payload.get("answer_reviewed") is True,
            answer_review_note=cls.string(payload, "answer_review_note"),
        )

    @staticmethod
    def string(payload: dict[str, object], key: str) -> str:
        value = payload.get(key)
        if not isinstance(value, str) or not value.strip():
            raise ValueError(f"sample field `{key}` should be a non-empty string")
        return value

    @staticmethod
    def string_list(payload: dict[str, object], key: str) -> list[str]:
        value = payload.get(key)
        if not isinstance(value, list):
            raise ValueError(f"sample field `{key}` should be an array")
        strings = [item for item in value if isinstance(item, str) and item.strip()]
        if len(strings) != len(value):
            raise ValueError(f"sample field `{key}` should contain only non-empty strings")
        return strings

    @staticmethod
    def path(payload: dict[str, object], root: pathlib.Path, key: str) -> pathlib.Path:
        raw_path = pathlib.Path(Sample.string(payload, key))
        return raw_path if raw_path.is_absolute() else root / raw_path


class Manifest:
    def __init__(self, path: pathlib.Path) -> None:
        self.path = path
        self.root = pathlib.Path.cwd()
        self.payload = json.loads(path.read_text(encoding="utf-8"))
        self.allowlist = self._allowlist()
        self.samples = self._samples()

    def validate(self, minimum_public: int, minimum_original: int) -> list[str]:
        errors: list[str] = []
        public_count = sum(1 for sample in self.samples if sample.kind == "public")
        original_count = sum(1 for sample in self.samples if sample.kind == "original")
        if public_count < minimum_public:
            errors.append(f"public sample count is {public_count}, expected {minimum_public}+")
        if original_count < minimum_original:
            errors.append(f"original sample count is {original_count}, expected {minimum_original}+")
        errors.extend(self._identity_errors())
        for sample in self.samples:
            errors.extend(self._sample_errors(sample))
        return errors

    def _identity_errors(self) -> list[str]:
        errors: list[str] = []
        ids: set[str] = set()
        inputs: dict[pathlib.Path, str] = {}
        answers: dict[pathlib.Path, str] = {}
        for sample in self.samples:
            if not re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]{0,127}", sample.id):
                errors.append(
                    f"{sample.id}: sample id must be a safe filename component"
                )
            if sample.id in ids:
                errors.append(f"{sample.id}: duplicate sample id")
            ids.add(sample.id)

            input_path = sample.input_path.resolve()
            answer_path = sample.answer_path.resolve()
            if input_path in inputs:
                errors.append(
                    f"{sample.id}: input path is also used by {inputs[input_path]}"
                )
            else:
                inputs[input_path] = sample.id
            if answer_path in answers:
                errors.append(
                    f"{sample.id}: answer path is also used by {answers[answer_path]}"
                )
            else:
                answers[answer_path] = sample.id
            if input_path == answer_path:
                errors.append(f"{sample.id}: input and answer paths must be distinct")

        for path in sorted(inputs.keys() & answers.keys()):
            errors.append(
                f"{inputs[path]}: input path is also an answer path for {answers[path]}"
            )
        return errors

    def _allowlist(self) -> set[str]:
        value = self.payload.get("license_allowlist")
        if not isinstance(value, list):
            raise SystemExit("manifest license_allowlist should be an array")
        return {str(item) for item in value}

    def _samples(self) -> list[Sample]:
        samples = self.payload.get("samples")
        if not isinstance(samples, list):
            raise SystemExit("manifest samples should be an array")
        loaded: list[Sample] = []
        for ordinal, payload in enumerate(samples, start=1):
            if not isinstance(payload, dict):
                raise SystemExit("manifest sample should be an object")
            loaded.append(Sample.from_payload(payload, self.root, ordinal))
        return loaded

    def _sample_errors(self, sample: Sample) -> list[str]:
        errors: list[str] = []
        if sample.kind not in {"public", "original"}:
            errors.append(f"{sample.id}: unknown kind {sample.kind}")
        if not sample.input_path.is_file():
            errors.append(f"{sample.id}: input file is missing")
        if not sample.answer_path.is_file():
            errors.append(f"{sample.id}: answer file is missing")
        if not sample.answer_path.name.endswith("_answer.md"):
            errors.append(f"{sample.id}: answer fixture should use xxx_answer.md naming")
        if sample.license not in self.allowlist:
            errors.append(f"{sample.id}: license {sample.license} is not allowlisted")
        if not re.fullmatch(r"[0-9a-f]{40}", sample.source_commit):
            errors.append(f"{sample.id}: source_commit should be a 40-character SHA")
        if not sample.answer_reviewed:
            errors.append(f"{sample.id}: answer fixture has not been reviewed")
        if sample.kind == "original":
            errors.extend(self._original_errors(sample))
        return errors

    def _original_errors(self, sample: Sample) -> list[str]:
        errors: list[str] = []
        if sample.input_path.is_file():
            content = sample.input_path.read_text(encoding="utf-8")
            if len(content) < 200:
                errors.append(f"{sample.id}: original sample is shorter than 200 characters")
        if len(sample.historical_patterns) < 2:
            errors.append(f"{sample.id}: original sample should combine two or more patterns")
        return errors


def execution_path(sample: Sample, root: pathlib.Path, *, answer: bool = False) -> pathlib.Path:
    suffix = "_answer.md" if answer else ".md"
    return root / f"{sample.execution_stem}{suffix}"
