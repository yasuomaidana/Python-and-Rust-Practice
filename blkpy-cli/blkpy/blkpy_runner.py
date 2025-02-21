from subprocess import CalledProcessError

import click

from lsblk_selector import run_lsblk

def blk_run(device:str,verbose:bool=False):
    try:
        print(f"{run_lsblk(device, verbose)}")
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