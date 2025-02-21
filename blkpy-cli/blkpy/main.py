import click

from .commands import info, nice

@click.group()
def main():
    pass


## Long way to do it
# @main.command()
# @click.option('--verbose', '-v', is_flag=True)
# @click.argument('device')
# def info(device, verbose):
#     print(f"Device: {device}")
#     print(f"Verbose: {verbose}")
#     blk_run(device, verbose)
main.add_command(info)
main.add_command(nice)