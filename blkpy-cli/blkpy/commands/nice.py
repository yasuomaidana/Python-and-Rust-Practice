import click

from blkpy.blkpy_runner import blk_run

@click.command(short_help='Get information about a device in a pretty way')
@click.argument('device')
def nice(device):
    print(f"Device: {device}")
    blk_run(device, True)