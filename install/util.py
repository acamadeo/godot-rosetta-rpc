import re
from pathlib import Path

AUTOLOAD_KEY = "RpcRuntime"

def read_autoload_value(project_godot: Path, section: str, key: str) -> str | None:
    lines = project_godot.read_text().splitlines()
    in_autoload = False
    pattern = re.compile(rf'^{re.escape(key)}="(.*)"$')
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            in_autoload = stripped == f"[{section}]"
            continue
        if in_autoload:
            match = pattern.match(stripped)
            if match:
                return match.group(1)
    return None


def set_config_value(project_godot: Path, section: str, key: str, value: str) -> None:
    """Inserts or replaces `key="value"` inside the `[section]` section."""
    lines = project_godot.read_text().splitlines(keepends=True)
    entry = f'{key}="{value}"\n'
    key_pattern = re.compile(rf"^{re.escape(key)}=")
    section_header = f"[{section}]"

    section_idx = next(
        (i for i, l in enumerate(lines) if l.strip() == section_header), None
    )

    # Create the section if not present. Trailing blank lines are stripped
    # first so repeated invocations don't accumulate a growing gap before the
    # new section header.
    if section_idx is None:
        while lines and lines[-1].strip() == "":
            lines.pop()
        if lines:
            if not lines[-1].endswith("\n"):
                lines[-1] += "\n"
            lines.append("\n")
        lines += [f"{section_header}\n\n", entry]
        project_godot.write_text("".join(lines))
        return

    section_end = next(
        (
            i
            for i in range(section_idx + 1, len(lines))
            if lines[i].strip().startswith("[") and lines[i].strip() != section_header
        ),
        len(lines),
    )
    existing_idx = next(
        (
            i
            for i in range(section_idx + 1, section_end)
            if key_pattern.match(lines[i].strip())
        ),
        None,
    )

    if existing_idx is not None:
        lines[existing_idx] = entry
    else:
        # Insert right after the section's last non-blank line, in order to
        # preserve any gap between this section and the next one.
        last_content_idx = next(
            (i for i in range(section_end - 1, section_idx, -1) if lines[i].strip()),
            section_idx,
        )
        lines.insert(last_content_idx + 1, entry)

    project_godot.write_text("".join(lines))


def remove_config_section(project_godot: Path, section: str) -> None:
    """Deletes the entire `[section]` block (header through its last content
    line before the next section or EOF), if present."""
    lines = project_godot.read_text().splitlines(keepends=True)
    section_header = f"[{section}]"

    section_idx = next(
        (i for i, l in enumerate(lines) if l.strip() == section_header), None
    )
    if section_idx is None:
        return

    section_end = next(
        (
            i
            for i in range(section_idx + 1, len(lines))
            if lines[i].strip().startswith("[") and lines[i].strip() != section_header
        ),
        len(lines),
    )

    del lines[section_idx:section_end]
    project_godot.write_text("".join(lines))
