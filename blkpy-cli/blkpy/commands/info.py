import click

from blkpy.blkpy_runner import blk_run


@click.command()
@click.option('--verbose', '-v', is_flag=True)
@click.argument('device')
def info(device, verbose):
    print(f"Device: {device}")
    blk_run(device, verbose)
