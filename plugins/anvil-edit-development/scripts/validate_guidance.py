"""Validate the repository-local Anvil Edit development guidance bundle."""

from __future__ import annotations

import json
import re
import sys
from html import unescape
from pathlib import Path
from urllib.parse import unquote, urlsplit


PLUGIN_ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = PLUGIN_ROOT.parents[1]
SKILLS_ROOT = PLUGIN_ROOT / "skills"
NAME_RE = re.compile(r"^[a-z0-9]+(?:-[a-z0-9]+)*$")
FRONTMATTER_RE = re.compile(r"\A---\r?\n(.*?)\r?\n---\r?\n", re.DOTALL)
AGENT_STRING_FIELD_RE = re.compile(r'^  ([a-z_]+):\s*("(?:[^"\\]|\\.)*")$')
AGENT_BOOL_FIELD_RE = re.compile(r"^  ([a-z_]+):\s*(true|false)$")
WINDOWS_ABSOLUTE_RE = re.compile(r"^[A-Za-z]:[\\/]")
REFERENCE_DEFINITION_RE = re.compile(r"^ {0,3}\[((?:\\.|[^\]])+)\]:\s*(.*)$")
FENCE_OPEN_RE = re.compile(r"^ {0,3}(`{3,}|~{3,})(.*)$")
FENCE_CLOSE_RE = re.compile(r"^ {0,3}(`{3,}|~{3,})[ \t]*$")
BLOCKQUOTE_RE = re.compile(r"^ {0,3}>[ \t]?")
LIST_MARKER_RE = re.compile(r"^ {0,3}(?:[-+*]|\d{1,9}[.)])[ \t]+")
AGENT_KEYS = {"display_name", "short_description", "default_prompt"}
AGENT_POLICY_KEYS = {"allow_implicit_invocation"}
EXTERNAL_SCHEMES = {"http", "https", "mailto"}
REQUIRED_SKILLS = {"refresh-product-guidance"}
RAW_DRIVE_RE = re.compile(r"(?<![A-Za-z0-9+.-])[A-Za-z]:[\\/]")
RAW_ABSOLUTE_TARGET_RE = re.compile(
    r"(?:\]\(|\]:)\s*<?(?:[\\/]|file:)", re.IGNORECASE
)
RAW_LINE_ABSOLUTE_RE = re.compile(r"^\s*<?(?:[\\/]|file:)", re.IGNORECASE)


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def parse_frontmatter(
    text: str, path: Path, errors: list[str]
) -> dict[str, str]:
    match = FRONTMATTER_RE.match(text)
    if not match:
        fail(errors, f"{path}: missing YAML frontmatter")
        return {}

    values: dict[str, str] = {}
    for line in match.group(1).splitlines():
        if not line.strip():
            continue
        if ":" not in line:
            fail(errors, f"{path}: invalid frontmatter line: {line}")
            continue
        key, value = line.split(":", 1)
        key = key.strip()
        if key in values:
            fail(errors, f"{path}: duplicate frontmatter key: {key}")
        values[key] = value.strip().strip('"').strip("'")
    return values


def validate_manifest(errors: list[str], plugin_root: Path = PLUGIN_ROOT) -> None:
    path = plugin_root / ".codex-plugin" / "plugin.json"
    try:
        manifest = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(errors, f"{path}: cannot parse plugin manifest: {exc}")
        return

    if not isinstance(manifest, dict):
        fail(errors, f"{path}: plugin manifest must be an object")
        return
    if manifest.get("name") != plugin_root.name:
        fail(errors, f"{path}: manifest name must match plugin directory")
    if manifest.get("skills") != "./skills/":
        fail(errors, f"{path}: skills must be ./skills/")
    if not re.fullmatch(r"\d+\.\d+\.\d+", str(manifest.get("version", ""))):
        fail(errors, f"{path}: version must be semantic x.y.z")
    interface = manifest.get("interface")
    if not isinstance(interface, dict):
        fail(errors, f"{path}: interface must be an object")
        return
    for key in ("displayName", "shortDescription", "longDescription"):
        if not isinstance(interface.get(key), str) or not interface[key].strip():
            fail(errors, f"{path}: missing interface.{key}")


def parse_agent_yaml(
    text: str, path: Path, errors: list[str]
) -> dict[str, str | bool]:
    """Parse the intentionally strict, dependency-free agent metadata subset."""

    lines = [line for line in text.splitlines() if line.strip()]
    if not lines or lines[0] != "interface:":
        fail(errors, f"{path}: first non-empty line must be interface:")
        return {}

    values: dict[str, str | bool] = {}
    sections: set[str] = set()
    section: str | None = None
    for line in lines:
        if "\t" in line:
            fail(errors, f"{path}: tabs are not allowed")
            continue

        if not line.startswith(" "):
            if line not in {"interface:", "policy:"}:
                fail(errors, f"{path}: unsupported root field: {line}")
                section = None
                continue
            section = line[:-1]
            if section in sections:
                fail(errors, f"{path}: duplicate root field: {section}")
            sections.add(section)
            continue

        if section == "interface":
            match = AGENT_STRING_FIELD_RE.fullmatch(line)
            if not match:
                fail(errors, f"{path}: invalid interface field or indentation: {line}")
                continue
            key, encoded_value = match.groups()
            try:
                value: str | bool = json.loads(encoded_value)
            except json.JSONDecodeError as exc:
                fail(errors, f"{path}: invalid quoted value for {key}: {exc.msg}")
                continue
        elif section == "policy":
            match = AGENT_BOOL_FIELD_RE.fullmatch(line)
            if not match:
                fail(errors, f"{path}: invalid policy field or indentation: {line}")
                continue
            key, encoded_value = match.groups()
            value = encoded_value == "true"
        else:
            fail(errors, f"{path}: field appears outside a supported section: {line}")
            continue

        if key in values:
            fail(errors, f"{path}: duplicate agent field: {key}")
            continue
        values[key] = value

    missing = AGENT_KEYS - values.keys()
    missing_policy = AGENT_POLICY_KEYS - values.keys()
    extra = values.keys() - AGENT_KEYS - AGENT_POLICY_KEYS
    if missing:
        fail(errors, f"{path}: missing interface fields: {', '.join(sorted(missing))}")
    if missing_policy:
        fail(errors, f"{path}: missing policy fields: {', '.join(sorted(missing_policy))}")
    if extra:
        fail(errors, f"{path}: unsupported agent fields: {', '.join(sorted(extra))}")
    for key in sorted(AGENT_KEYS & values.keys()):
        value = values[key]
        if not isinstance(value, str) or not value.strip():
            fail(errors, f"{path}: interface field must not be blank: {key}")
    if values.get("allow_implicit_invocation") is not False:
        fail(
            errors,
            f"{path}: policy.allow_implicit_invocation must be false for "
            "explicit-only loading",
        )
    return values


def validate_skills(errors: list[str], skills_root: Path = SKILLS_ROOT) -> int:
    if not skills_root.is_dir():
        fail(errors, f"{skills_root}: missing skills directory")
        return 0
    directories = sorted(path for path in skills_root.iterdir() if path.is_dir())
    missing_required = REQUIRED_SKILLS - {path.name for path in directories}
    if missing_required:
        fail(
            errors,
            f"{skills_root}: missing required skills: {', '.join(sorted(missing_required))}",
        )
    count = 0
    for directory in directories:
        count += 1
        skill_path = directory / "SKILL.md"
        agent_path = directory / "agents" / "openai.yaml"
        if not skill_path.is_file():
            fail(errors, f"{directory}: missing SKILL.md")
            continue

        try:
            text = skill_path.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as exc:
            fail(errors, f"{skill_path}: cannot read skill metadata: {exc}")
            continue
        values = parse_frontmatter(text, skill_path, errors)
        if set(values) != {"name", "description"}:
            fail(errors, f"{skill_path}: frontmatter must contain only name and description")
        if values.get("name") != directory.name or not NAME_RE.fullmatch(directory.name):
            fail(errors, f"{skill_path}: skill name must match its hyphen-case directory")
        if not values.get("description"):
            fail(errors, f"{skill_path}: description is required")
        if len(text.splitlines()) > 500:
            fail(errors, f"{skill_path}: exceeds 500 lines")
        if "[TODO" in text or "TODO:" in text:
            fail(errors, f"{skill_path}: contains placeholder text")

        if not agent_path.is_file():
            fail(errors, f"{directory}: missing agents/openai.yaml")
            continue
        try:
            agent_text = agent_path.read_text(encoding="utf-8")
        except (OSError, UnicodeError) as exc:
            fail(errors, f"{agent_path}: cannot read agent metadata: {exc}")
            continue
        agent_values = parse_agent_yaml(
            agent_text, agent_path, errors
        )
        short_description = agent_values.get("short_description", "")
        if (
            not isinstance(short_description, str)
            or not 25 <= len(short_description) <= 64
        ):
            fail(errors, f"{agent_path}: short_description must be 25-64 characters")
        default_prompt = agent_values.get("default_prompt", "")
        if (
            not isinstance(default_prompt, str)
            or f"${directory.name}" not in default_prompt
        ):
            fail(errors, f"{agent_path}: default_prompt must name ${directory.name}")
    return count


def markdown_target(raw_target: str, path: Path, errors: list[str]) -> str | None:
    """Return a link target while accepting an optional quoted Markdown title."""

    raw_target = raw_target.strip()
    if raw_target.startswith("<"):
        closing = raw_target.find(">")
        if closing < 0:
            fail(errors, f"{path}: unterminated angle-bracket link: {raw_target}")
            return None
        target = raw_target[1:closing]
        remainder = raw_target[closing + 1 :].strip()
    else:
        match = re.fullmatch(
            r"(\S+?)(?:\s+(?:\"[^\"]*\"|'[^']*'|\([^)]*\)))?", raw_target
        )
        if not match:
            fail(errors, f"{path}: unsupported local link syntax: {raw_target}")
            return None
        target = match.group(1)
        remainder = ""

    if remainder and not re.fullmatch(r'(?:"[^"]*"|\'[^\']*\'|\([^)]*\))', remainder):
        fail(errors, f"{path}: invalid Markdown link title: {raw_target}")
        return None
    return target


def is_within(path: Path, root: Path) -> bool:
    try:
        path.relative_to(root)
        return True
    except ValueError:
        return False


def find_balanced(
    text: str,
    start: int,
    opening: str,
    closing: str,
    *,
    protect_destination_literals: bool = False,
) -> int | None:
    depth = 0
    index = start
    quote: str | None = None
    angle = False
    while index < len(text):
        character = text[index]
        if character == "\\":
            index += 2
            continue
        if protect_destination_literals:
            if quote is not None:
                if character == quote:
                    quote = None
                index += 1
                continue
            if angle:
                if character == ">":
                    angle = False
                index += 1
                continue
            if (
                character in {'"', "'"}
                and index > start
                and text[index - 1].isspace()
            ):
                quote = character
                index += 1
                continue
            if character == "<":
                angle = True
                index += 1
                continue
        if character == opening:
            depth += 1
        elif character == closing:
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return None


def normalize_reference_label(label: str) -> str:
    label = re.sub(r"\\([\\`*{}\[\]()#+\-.!_>])", r"\1", label)
    return " ".join(label.split()).casefold()


def strip_container_markers(line: str) -> str:
    """Remove explicit block-quote and list containers for link parsing."""

    while True:
        blockquote = BLOCKQUOTE_RE.match(line)
        if blockquote:
            line = line[blockquote.end() :]
            continue
        list_marker = LIST_MARKER_RE.match(line)
        if list_marker:
            line = line[list_marker.end() :]
            continue
        return line


def escaped_at(text: str, index: int) -> bool:
    backslashes = 0
    index -= 1
    while index >= 0 and text[index] == "\\":
        backslashes += 1
        index -= 1
    return backslashes % 2 == 1


def extract_markdown_targets(text: str, path: Path, errors: list[str]) -> list[str]:
    """Extract inline, image, and reference-style Markdown destinations."""

    targets: list[str] = []
    definitions: dict[str, str] = {}
    reference_uses: list[str] = []
    visible_lines: list[str] = []
    fence: tuple[str, int] | None = None

    for original_line in text.splitlines():
        line = strip_container_markers(original_line)
        if fence is not None:
            close = FENCE_CLOSE_RE.match(line)
            if close:
                marker = close.group(1)
                if marker[0] == fence[0] and len(marker) >= fence[1]:
                    fence = None
            continue
        opening = FENCE_OPEN_RE.match(line)
        if opening:
            marker = opening.group(1)
            fence = (marker[0], len(marker))
            continue
        visible_lines.append(line)
        definition = REFERENCE_DEFINITION_RE.match(line)
        if definition:
            label = normalize_reference_label(definition.group(1))
            if label in definitions:
                fail(errors, f"{path}: duplicate reference definition: {label}")
            definitions[label] = definition.group(2).strip()

    for line in visible_lines:
        index = 0
        while index < len(line):
            if line[index] == "`":
                run = 1
                while index + run < len(line) and line[index + run] == "`":
                    run += 1
                closing = line.find("`" * run, index + run)
                index = len(line) if closing < 0 else closing + run
                continue
            if line[index] != "[":
                index += 1
                continue
            if escaped_at(line, index):
                index += 1
                continue

            label_end = find_balanced(line, index, "[", "]")
            if label_end is None:
                fail(errors, f"{path}: unterminated Markdown label")
                break
            label = line[index + 1 : label_end]
            next_index = label_end + 1
            if next_index < len(line) and line[next_index] == "(":
                target_end = find_balanced(
                    line,
                    next_index,
                    "(",
                    ")",
                    protect_destination_literals=True,
                )
                if target_end is None:
                    fail(errors, f"{path}: unterminated inline link destination")
                    break
                targets.append(line[next_index + 1 : target_end])
                index = target_end + 1
                continue
            if next_index < len(line) and line[next_index] == "[":
                reference_end = find_balanced(line, next_index, "[", "]")
                if reference_end is None:
                    fail(errors, f"{path}: unterminated reference link label")
                    break
                reference = line[next_index + 1 : reference_end] or label
                reference_uses.append(normalize_reference_label(reference))
                index = reference_end + 1
                continue
            index = label_end + 1

    for label, raw_target in definitions.items():
        if not raw_target:
            fail(errors, f"{path}: empty reference definition: {label}")
        else:
            targets.append(raw_target)
    for label in reference_uses:
        if label not in definitions:
            fail(errors, f"{path}: missing reference definition: {label}")
    return targets


def validate_raw_containment(text: str, path: Path, errors: list[str]) -> bool:
    """Fail closed on path escapes even when Markdown syntax is unfamiliar."""

    found = False
    decoded = unescape(unquote(text))
    for line_number, line in enumerate(decoded.splitlines(), start=1):
        reasons: list[str] = []
        if re.search(r"\.\.[\\/]", line):
            reasons.append("parent traversal")
        if RAW_DRIVE_RE.search(line):
            reasons.append("drive-qualified path")
        if RAW_ABSOLUTE_TARGET_RE.search(line):
            reasons.append("filesystem-absolute destination")
        if RAW_LINE_ABSOLUTE_RE.search(line):
            reasons.append("line-leading filesystem destination")
        if reasons:
            found = True
            fail(
                errors,
                f"{path}:{line_number}: repository Markdown forbids "
                + ", ".join(reasons),
            )
    return found


def validate_local_links(
    errors: list[str], repo_root: Path = REPO_ROOT, files: list[Path] | None = None
) -> int:
    repo_root = repo_root.resolve()
    if files is None:
        files = [repo_root / "README.md", repo_root / "AGENTS.md"]
        files.extend((repo_root / "docs").rglob("*.md"))
        files.extend((repo_root / "plugins").rglob("SKILL.md"))

    checked = 0
    for path in files:
        text = path.read_text(encoding="utf-8")
        raw_containment_found = validate_raw_containment(text, path, errors)
        for raw_target in extract_markdown_targets(text, path, errors):
            target = markdown_target(raw_target, path, errors)
            if target is None or target.startswith("#"):
                continue
            parsed = urlsplit(target)
            if parsed.scheme:
                if parsed.scheme.lower() not in EXTERNAL_SCHEMES:
                    if not raw_containment_found:
                        fail(errors, f"{path}: unsupported or local URL scheme: {target}")
                continue
            if (
                target.startswith(("/", "\\"))
                or WINDOWS_ABSOLUTE_RE.match(target)
                or target.startswith("//")
            ):
                if not raw_containment_found:
                    fail(errors, f"{path}: absolute local links are not allowed: {target}")
                continue

            local_path = unescape(unquote(parsed.path))
            if not local_path:
                continue
            checked += 1
            resolved = (path.parent / local_path).resolve()
            if not is_within(resolved, repo_root):
                if not raw_containment_found:
                    fail(errors, f"{path}: local link escapes repository: {target}")
            elif not resolved.exists():
                fail(errors, f"{path}: broken local link: {target}")
    return checked


def main() -> int:
    errors: list[str] = []
    validate_manifest(errors)
    skill_count = validate_skills(errors)
    link_count = validate_local_links(errors)
    if errors:
        for error in errors:
            print(f"ERROR: {error}", file=sys.stderr)
        print(f"Guidance validation failed with {len(errors)} error(s).", file=sys.stderr)
        return 1
    print(f"Guidance validation passed: {skill_count} skills, {link_count} local links.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
