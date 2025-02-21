FROM python:3.10-alpine

# Install lsblk
RUN apk add --no-cache util-linux

WORKDIR /usr/src/app

# Install dependencies
COPY requirements.txt .
#COPY *.py ./

RUN pip install -r requirements.txt

# Uses sh (the shell) to execute the command.
# Runs python setup.py develop, which typically installs the package in development mode.
# After the setup script completes, it keeps the shell open (&& sh), allowing for interactive use or further commands.
# The -c option in the ENTRYPOINT instruction specifies that the following string is a command to be executed by the shell (sh). 
# In this case, it allows the shell to run the command python setup.py develop && sh.
#ENTRYPOINT ["sh", "-c", "python setup.py develop && sh"] old way
ENTRYPOINT ["sh", "-c", "pip install -e .  && sh"]