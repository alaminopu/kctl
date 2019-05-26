# kctl

[![Build Status](https://travis-ci.com/alaminopu/kctl.svg?branch=master)](https://travis-ci.com/alaminopu/kctl)

A CLI wrapper for making kubernetes commands much easier

## Usage

```
$ kctl --help
USAGE:
    kctl [OPTIONS] <command> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --namespace <namespace>    Specify the namespace to work on [default: default]

ARGS:
    <command>    Input command you want to run! [possible values: pod, svc, deploy, log, exec, forward, set-
                 namespace]
    <app>        Get specific app pods
    <port>       Get port number for port forwarding
```

[Usage Docs](https://github.com/alaminopu/kctl/wiki)  

## Installation

**Homebrew**
```
brew tap alaminopu/kctl https://github.com/alaminopu/kctl.git  
brew install kctl
```

**Manual**

Before building the tool, install `rust` and `cargo`

On Linux and macOS systems, this is done as follows:

```
curl https://sh.rustup.rs -sSf | sh
```

Then run

```
cargo build --release 
```

You will find the binary in /target folder

Update permission 

```
chmod a+x kctl
```

Move the binary to `/usr/local/bin` to make it executable from any terminal window 

```
mov kctl /usr/local/bin
```


## Contributing
- Fork the repo
- Create an issue in the issue section
- Write code and commit changes 
- Send an PR to the master branch 


## Authors

* **Md. Al-Amin** [alaminopu](https://github.com/alaminopu)
* **Arvind Thangamani** [dnivra26](https://github.com/dnivra26)

See also the list of [contributors](https://github.com/alaminopu/kctl/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details