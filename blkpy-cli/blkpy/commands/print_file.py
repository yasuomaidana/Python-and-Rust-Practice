import sys

import click


@click.command(name="printf", short_help='Print a file')
@click.argument('input', type=click.File('r'), default=sys.stdin)
@click.pass_context
def print_file(_ctx, input):
    """
    Print a file

    INPUT: File to print
    \f

    :param str input: File to print
    """
    click.echo(input.read())