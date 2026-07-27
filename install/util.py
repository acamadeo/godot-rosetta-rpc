import re
from pathlib import Path

AUTOLOAD_KEY = "RpcRuntime"

def read_autoload_value(project_godot: Path, key: str) -> str | None:
    lines = project_godot.read_text().splitlines()
    in_autoload = False
    pattern = re.compile(rf'^{re.escape(key)}="(.*)"$')
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            in_autoload = stripped == "[autoload]"
            continue
        if in_autoload:
            match = pattern.match(stripped)
            if match:
                return match.group(1)
    return None
