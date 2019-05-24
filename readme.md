# kctl

A CLI wrapper for making kubernetes commands much easier

## Build

Before building the tool, install `rust` and `cargo`

On Linux and macOS systems, this is done as follows:

```
curl https://sh.rustup.rs -sSf | sh
```



## Installation

**Homebrew**
```
brew tap alaminopu/kctl https://github.com/alaminopu/kctl.git  
brew install kctl
```

**Manual**

Run

```
cargo build --release 
```

You will find the binary in /target folder

```
chmod a+x kctl
```

```
mov kctl /usr/local/bin
```



## Kubernetes commands 
1. Set a default namespace 

     ```
    kctl set-namespace -n <namespace>
    ```

    Example:

     ```
    kctl set-namespace -n food
    ```

2. Getting all pods of your defined namespace.

    ```
    kctl pod
    ```

    Search specific pods with app name

     ```
    kctl pod <app-name>
    ```

3. Getting all services of your defined namespace.
    
    ```
    kctl svc 
    ```

    Search specific service with app name

     ```
    kctl svc <app-name>
    ```

4. Getting all deployments of your defined namespace.

    ```
    kctl deploy 
    ```

    Search specific deploy with app name

     ```
    kctl deploy <app-name>
    ```


5. Checking logs of any of the deployment or app of your defined namespace.

    ```
    kctl logs <app-name>
    ```

    Example: 

    ```
    kctl logs search
    ```

6. Port forward any of the deployment or app of your defined namespace.

    ```
    kctl forward <app-name> <port-number>
    ```

    Example: 

    ```
    kctl forward search 8030
    ```

    or 

    ```
    kctl forward search 8030:8030
    ```

7. Exec to any pod of the deployment or app of your defined namespace. By default it will try to use `bash`.

     ```
    kctl exec <app-name>
    ```

    Example: 

    ```
    kctl exec search
    ```

## Arguments
1. Namespace  
    Default namespace is "default". Change it by specifying optional argument -n or --namespace

    Example: 

    ```
    kctl pod -n resto
    ```


## Contributing
- Fork the repo
- Create an issue from in the issue section
- Write code and commit changes 
- Send an PR to the master branch 


## Authors

* **Md. Al-Amin** [alaminopu](https://github.com/alaminopu)
* **Arvind Thangamani** [dnivra26](https://github.com/dnivra26)

See also the list of [contributors](https://github.com/alaminopu/kctl/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details