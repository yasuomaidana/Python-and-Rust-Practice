import click

from blkpy.blkpy_runner import blk_run

@click.command()
@click.argument('device')
def nice(device):
    print(f"Device: {device}")
    blk_run(device, True)