from setuptools import setup, find_packages

with open('requirements.txt') as f:
    requirements = f.read().splitlines()

setup(
    name='blkpy-demo',
    description='A demo package for blkpy to list block devices',
    packages=find_packages(),
    author='Yo el mero mero',
    entry_points = """
        [console_scripts]
        blkpy = blkpy.main:main
    """,
    install_requires=requirements,
    version='0.1.0',   
)