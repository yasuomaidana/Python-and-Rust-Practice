from subprocess import CalledProcessError
from lsblk_selector import run_lsblk
import logging

log = logging.getLogger(__name__)

def blk_run(device: str, verbose: bool = False):
    try:
        print(f"{run_lsblk(device, verbose)}")
    except CalledProcessError as e:
        log.error(f"Sorry, couldn't run the code because of: \n\t{e}")
    except FileNotFoundError as e:
        if "lsblk" in str(e):
            log.error("Sorry, couldn't find the lsblk command. Make sure it is installed 😒.")
            # click.echo(click.style(
            #     f"Sorry, couldn't find the lsblk command. Make sure it is installed 😒.",
            #     fg='red'))
        else:
            log.info(f"Sorry, couldn't find the file: \n\t{e}")
        if verbose:
            log.error(e)
    except Exception as e:
        log.error(f"Sorry, something went wrong. IDK and IDC.{e}, type: {type(e)}")
