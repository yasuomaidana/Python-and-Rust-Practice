import click
import datetime
import logging
from blkpy.commands import info, nice, show_time, print_file
from blkpy.log_formatter import CustomFormatter

CONTEXT_SETTINGS = dict(help_option_names=['-h', '--help'])
# log = logging.getLogger(__name__)
log = logging.getLogger("cli-root")

# Define format for logs
fmt = '%(asctime)s | %(levelname)8s | %(message)s'

# Create stdout handler for logging to the console (logs all five levels)
stdout_handler = logging.StreamHandler()
stdout_handler.setFormatter(CustomFormatter(fmt))

# Create file handler for logging to a file (logs all five levels)
today = datetime.date.today()
file_handler = logging.FileHandler('my_app_{}.log'.format(today.strftime('%Y_%m_%d')))
file_handler.setLevel(logging.DEBUG)
file_handler.setFormatter(logging.Formatter(fmt))

# Add both handlers to the logger
log.addHandler(stdout_handler)
log.addHandler(file_handler)

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

    if debug_level == 0:
        logging.basicConfig(level=logging.DEBUG)
    elif debug_level == 1:
        logging.basicConfig(level=logging.INFO)
    elif debug_level == 2:
        logging.basicConfig(level=logging.WARNING)
    elif debug_level == 3:
        logging.basicConfig(level=logging.ERROR)
    if verbose:
        log.debug("Verbose mode enabled")


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
main.add_command(print_file)

if __name__ == '__main__':
    main()
