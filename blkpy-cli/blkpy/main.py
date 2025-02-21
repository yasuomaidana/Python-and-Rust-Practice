import click

from .commands import info, nice, show_time

CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])


# context settings also works with command
@click.group(
    context_settings=CONTEXT_SETTINGS,
    epilog='Check out our docs at ... this is not a novel get out from here')
@click.option('--verbose', '-v', is_flag=True, help="Enable verbose mode")
# @click.option('--debug-level', '-d', type=click.Choice(['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']), help="Set debug level"
#               , default='INFO')
@click.option('--debug-level', '-d',
              envvar='DEBUG_LEVEL',
              type=click.IntRange(0, 3), help="Set debug level optionally with env var DEBUG_LEVEL"
              , default=0)
@click.pass_context
def main(ctx, verbose, debug_level):
    ctx.ensure_object(dict)
    ctx.obj.update(locals())
    print(f"Debug level: {debug_level}")
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
main.add_command(show_time)
