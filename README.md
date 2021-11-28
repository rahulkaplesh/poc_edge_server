For building up the rust dev environment for raspberry pi on ubuntu run the following commands :

1. rustup target add armv7-unknown-linux-gnueabihf - this will include rust buildchains for arm7 compilation
2. sudo apt install gcc-arm-linux-gnueabihf - install gcc libc linking for arm7 
3. Run the script 'deploy.sh' for building and deploying using rsync ... Check the ip address and user on the script ... at line 10

Currently tested this for arm7 install on raspberry pi 4

Suggestions : 

- Setting up ssh-keys between pi and ubuntu using 
    1. ssh-keygen
    2.  ssh-copy-id -i ~/.ssh/id_rsa.pub user-pi@ipaddress-pi