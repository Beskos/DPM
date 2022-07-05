## Table Of Contents
- [Table Of Contents](#table-of-contents)
- [About](#about)
- [Installation](#installation)
- [Usage](#usage)
- [Supported Distros](#supported-distros)
- [FAQ](#faq)
- [Donate](#donate)
  
## About
D(istro)P(ackage)M(anager) is meant for new linux users and/or distro hoppers.
DPM is a wrapper for your distro's package manager, so you don't have to remenber 
the syntax of each package manager. It is coded in Rust language, from a not Rust developer :) . 
You just call DPM on any distro, and it executes the corresponding command for your package manager.
	
## Installation
To install DPM, go to releases and download the latest version. Then open a terminal and execute:

```
$ cd ~/Downloads
$ sudo cp dpm /usr/bin/dpm
$ sudo chmod a+x dpm
```
That was it! Now You can use DPM from terminal just by its name.
Be careful to download DPM only from trusted sources to avoid any potential unwanted behavior.

## Usage
DPM can perform some basic package manager actions on most linux distros.
The syntax is simple, and self explanatory, 
* dpm install app1 app2 ...
* dpm remove app1 app2 ...
* dpm refresh
* dpm update
* dpm autoremove
* dpm search app1

**dpm install**: Installs the package(s) that follow after the install command, seperated by space.<br />
**dpm remove**:  Removes the package(s) that follow after the remove command, seperated by space.<br />
**dpm refresh**: It gets a list of the packages that are updatable, but does not perform the update. Depending on the distribution, It may download the new packages.<br />
**dpm update**: Updates all the updatable packages. If they are not already downloaded, it downloads them and then installs them.<br />
**dpm autoremove**: It removes any dependencies that wre installed for a package that has been removed and are no longer needed.<br />
**dpm search**: It search for packages with the name typed by user.<br />


## Supported Distros
Theoretically any debian-based, arch-based, fedora-based distribution should be compatible with DPM,
but that is not tested. Here is a list of tested and supported distributions:
* Ubuntu
* Debian
* Zorin OS
* Pop!OS
* Deepin
* Elementary OS
* Linux Mint
* CentOS
* Fedora
* Rocky linux
* Manjaro
* Arch
* EndeavourOS
* Garuda
* KDE Neon
* Alpine

DPM may be compatible with linux distributions that are not in the list. If so, please contact me to update the list.
## FAQ
**Q:** Is DPM something like snaps or flatpaks?<br />
**A:** No it is not. DPM is not a package manager and it does not offer cross-distro apps. It uses your distro's package manager

##
**Q:** Why should I use it?<br />
**A:** If you are a new linux user and you think that package manager has difficult syntax, or if you have to do with a lot of distros, then DPM will provide you an easy and cross-distro syntax to manage your packages.
##

##
**Q:** Does it require root privileges?<br />
**A:** Probaly Yes. It depends on your distro, but in the most cases, it will need sudo privileges. And that's the main reason you need to be sure where you download DPM from. You don't want some random program to get root access, that's for sure.
##

##
**Q:** Why do I have to copy a  file in order to install DPM? Why not include it in the PATH ?<br />
**A:** A new/not advanced user, does not even know what the PATH is. Copying an executable where other executables are may be more newbie frendly.
##

##
**Q:** Since you are not a Rust developer, why you use Rust?<br />
**A:** For being memory-safe and of course for speed. Okay you got me, for fun too.
##

## Donate
If you like DPM project, share it, contribute, and [buy me a beer!](https://www.paypal.com/donate?hosted_button_id=J8CU7S9BJN9E8)