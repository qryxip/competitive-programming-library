#!/usr/bin/env python3

import json
import subprocess
import sys
from argparse import ArgumentParser
from pathlib import Path
from subprocess import PIPE
from typing import Any, Dict, List, Optional, Tuple


def main() -> None:
    parser = ArgumentParser()

    subparsers = parser.add_subparsers(required=True)

    parser_compile = subparsers.add_parser('compile')
    parser_compile.add_argument('path', metavar='PATH')

    parser_execute = subparsers.add_parser('execute')
    parser_execute.add_argument('path', metavar='PATH')

    parser_list_dependencies = subparsers.add_parser('list-dependencies')
    parser_list_dependencies.add_argument('path', metavar='PATH')

    args = parser.parse_args()

    if sys.argv[1] == 'compile':
        language_compile(Path(args.path).resolve())
    elif sys.argv[1] == 'execute':
        command = language_get_executable_command(Path(args.path).resolve())
        subprocess.run(command, check=True)
    elif sys.argv[1] == 'list-dependencies':
        dependencies = language_list_dependencies(Path(args.path).resolve())
        print('\n'.join(map(str, dependencies)))
    else:
        raise Exception('should be unreachable')


def language_compile(path: Path) -> None:
    metadata = cargo_metadata(path.parent, True)
    target = find_target(metadata, path)
    if not target:
        raise Exception(f'{path} is not a main source file of any target')
    _, target = target
    if target['kind'] != ['bin']:
        raise Exception(f'`{target["name"]}` is not a `bin` target')
    subprocess.run(
        ['cargo', 'build', '--release', '--bin', target['name']],
        check=True, cwd=path.parent,
    )


def language_get_executable_command(path: Path) -> List[str]:
    metadata = cargo_metadata(path.parent, True)
    target = find_target(metadata, path)
    if not target:
        raise Exception(f'{path} is not a main source file of any target')
    _, target = target
    if target['kind'] != ['bin']:
        raise Exception(f'`{target["name"]}` is not a `bin` target')
    return [str(Path(metadata['target_directory'], 'release', target['name']))]


def language_list_dependencies(path: Path) -> List[Path]:
    for parent in path.parents:
        if parent.parent.joinpath('Cargo.toml').exists() and \
                parent.parts[-1] == 'target':
            print(
                f'Nope! This is a generated file, and is gitignored!: {path}',
                file=sys.stderr, flush=True,
            )
            return []
    ret = [other for other in path.parent.rglob('*.rs') if other != path]
    metadata = cargo_metadata(path.parent, False)
    target = find_target(metadata, path)
    if not target:
        return ret
    package, target = target
    packages_by_id = {package['id']: package
                      for package in metadata['packages']}
    return sorted(
        Path(target['src_path'])
        for node in metadata['resolve']['nodes']
        if node['id'] == package['id']
        for dep in node['deps']
        if not packages_by_id[dep['pkg']]['source'] and
        any(not dep_kind['kind'] or dep_kind['kind'] == 'build'
            for dep_kind in dep['dep_kinds'])
        for target in packages_by_id[dep['pkg']]['targets']
        if target['kind'] == ['lib'] and Path(target['src_path']) != path
    )


def cargo_metadata(cwd: Path, no_deps: bool) -> Dict[str, Any]:
    args = ['cargo', 'metadata', '--format-version', '1']
    if no_deps:
        args.append('--no-deps')
    return json.loads(subprocess.run(
        args, check=True, cwd=cwd, stdout=PIPE,
    ).stdout.decode())


def find_target(metadata: Dict[str, Any], src_path: Path,
                ) -> Optional[Tuple[Dict[str, Any], Dict[str, Any]]]:
    for package in metadata['packages']:
        for target in package['targets']:
            if Path(target['src_path']) == src_path:
                return package, target
    return None


if __name__ == '__main__':
    main()
