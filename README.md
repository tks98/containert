# containert

## Overview

containert serves as a simple container runtime tool designed for educational purposes. It executes binary files in secure, isolated settings using Linux cgroups and namespaces. 


## Usage

Run the program with the following command:

```bash
containert --binary_path [path/to/binary] --rootfs [path/to/rootfs]
```

- `binary_path`: Path to the binary you want to run in the container.
- `rootfs`: Path to the root filesystem for the container.

Example: (I extracted the rootfs for an ubuntu docker image)

```bash
containert --binary_path /bin/bash --rootfs ubuntu/rootfs
root@ubuntu-linux-22-04-desktop:/# ls
bin  boot  dev  etc  home  lib  lost+found  media  mnt  opt  proc  root  run  sbin  snap  srv  swap.img  sys  tmp  usr  var
```

## Contributing

Feel free to create issues for bugs and feature requests, or make pull requests to improve the utility.

## License

This project is licensed under the MIT License.
