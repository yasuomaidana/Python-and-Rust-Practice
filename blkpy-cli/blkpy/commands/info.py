import click

from blkpy.blkpy_runner import blk_run


@click.command(short_help='Get information about a device')
# @click.option('--verbose', '-v', is_flag=True)
@click.argument('device', required=False)
@click.pass_context
def info(ctx, device:str):
    """
    Print device information 
    
    DEVICE: Device name to get information about
    \f
    
    :param str device: Device name doesn't appear
    """
    print(f"Device: {device}")
    verbose = ctx.obj['verbose']
    blk_run(device, verbose)
