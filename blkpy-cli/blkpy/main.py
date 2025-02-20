from subprocess import CalledProcessError

import click
from lsblk_selector import run_lsblk


@click.command()
@click.option('--verbose', '-v', is_flag=True)
@click.argument('device')
def main(device, verbose):
    print(f"Device: {device}")
    print(f"Verbose: {verbose}")
    try:
        print(f"{run_lsblk(device)}")
    except CalledProcessError as e:
        print(f"Sorry couldn't run the code because of: \n\t{e}")
    except FileNotFoundError as e:
        if "lsblk" in str(e):
            click.echo(click.style(
                f"Sorry, couldn't find the lsblk command. Make sure it is installed 😒.",
                fg='red'))
        else:
            print(f"Sorry, couldn't find the file: \n\t{e}")
        if verbose:
            raise e
    except Exception as e:
        print(f"Sorry, something went wrong. IDK and IDC.{e}, type: {type(e)}")
