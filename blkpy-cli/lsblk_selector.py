import subprocess
import shlex
import json
from pprint import pprint


# create a function that runs subprocess and returns the output
def run_command(command):
    cmd = shlex.split(command)
    output = subprocess.check_output(cmd)
    return output

def run_lsblk(device:str, verbose:bool=False):
    """
    Runs lsblk command and produces JSON output:

    lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT
    {
    "blockdevices": [
        {"name": "vda", "size": "59.6G", "type": "disk", "mountpoint": null,
            "children": [
                {"name": "vda1", "size": "59.6G", "type": "part", "mountpoint": "/etc/hosts"}
            ]
        }
    ]
    }
    """
    command = f'lsblk -J -o NAME,SIZE,TYPE,MOUNTPOINT'
    output = run_command(command)
    devices = json.loads(output)['blockdevices']
    for parent in devices:
        if parent['name'] == device:
            if verbose:
                return pprint(parent)
            return parent
        for child in parent.get('children', []):
            if child['name'] == device:
                if verbose:
                    return pprint(child)
                return child


def main(device):
    print(f"         '{run_lsblk(device)}'")

if __name__ == '__main__':
    import sys
    print(sys.argv)
    main(sys.argv[-1])
    