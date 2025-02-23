from datetime import datetime as time

import click


@click.command(name="time", short_help='Show current time in a given format')
@click.option('--format', '-f', 'time_format',
              envvar='TIME_FORMAT',
              type=click.Choice(['default', 'rfc3339']),
              help="Defines the format of the output"
    , default='default')
@click.pass_context
def show_time(_ctx, time_format):
    if time_format == 'default':
        print(time.now().strftime("%Y-%m-%d %H:%M:%S"))
    else:
        print(time.now().isoformat())
