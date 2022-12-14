from omnipytent import *
from omnipytent.ext.idan import *

import json


@task
def check(ctx):
    cargo['check', '-q'] & ERUN.bang


@task
def build(ctx):
    cargo['build'][
        '--features', 'bevy/dynamic',
    ] & TERMINAL_PANEL.size(20)


@task
def run(ctx):
    cargo['run'][
        '--features', 'bevy/dynamic',
    ].with_env(
        RUST_LOG='mix_n_mech=info,bevy_yoleck=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task.options(alias=':1')
def level(ctx):
    ctx.key(lambda level: level['filename'].removesuffix('.yol').replace('_', ' '))
    ctx.value(lambda level: level['filename'].removesuffix('.yol'))
    with local.path('assets/levels/index.yoli').open() as f:
        level_index = json.load(f)
    for level in level_index[1]:
        yield level


@task
def execute(ctx, level=level):
    cargo['run'][
        '--features', 'bevy/dynamic',
        '--', '--level', level,
    ].with_env(
        RUST_LOG='mix_n_mech=info,bevy_yoleck=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def go(ctx):
    cargo['run'][
        '--features', 'bevy/dynamic',
        '--', '--editor',
    ].with_env(
        RUST_LOG='mix_n_mech=info',
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def clean(ctx):
    cargo['clean'] & BANG


@task
def launch_wasm(ctx):
    cargo['run'][
        '--target', 'wasm32-unknown-unknown'
    ].with_env(
        RUST_BACKTRACE='1',
    ) & TERMINAL_PANEL.size(20)


@task
def browse_wasm(ctx):
    local['firefox']['http://127.0.0.1:1334']()


@task
def clippy(ctx):
    cargo['clippy'] & ERUN.bang


@task
def cargo_fmt_run(ctx):
    cargo['fmt'] & ERUN.bang


@task
def erase_save(ctx):
    save_dir = local.path('~/.local/share/mixnmech')
    save_dir.delete()


@task
def move_assets_from_origs(ctx):
    suffixes_to_move = {
        '.png',
    }
    origs_dir = local.path('origs-for-assets')
    assets_dir = local.path('assets')
    for file in origs_dir.walk():
        if file.suffix not in suffixes_to_move:
            continue
        file.move(assets_dir / file.relative_to(origs_dir))
