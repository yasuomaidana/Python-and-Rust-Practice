import click

from blkpy.blkpy_runner import blk_run

@click.group()
def main():
    pass

@main.command()
@click.option('--verbose', '-v', is_flag=True)
@click.argument('device')
def info(device, verbose):
    print(f"Device: {device}")
    print(f"Verbose: {verbose}")
    blk_run(device, verbose)

@main.command()
@click.argument('device')
def nice(device):
    print(f"Device: {device}")
    blk_run(device, True)