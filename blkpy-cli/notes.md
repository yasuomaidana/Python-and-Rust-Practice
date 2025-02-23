# Notes

## Setup new commands

| setup.py Command     | New Command                                                        |
|----------------------|--------------------------------------------------------------------|
| setup.py sdist       | python -m build (with build)                                       |
| setup.py bdist_wheel | python -m build (with build)                                       |
| setup.py test        | pytest (usually via tox or nox)                                    |
| setup.py install     | pip install                                                        |
| setup.py develop     | pip install -e                                                     |
| setup.py upload      | twine upload (with twine)                                          |
| setup.py check       | twine check (this doesn't do all the same checks but it's a start) |
| Custom commands      | tox and nox environments.                                          |