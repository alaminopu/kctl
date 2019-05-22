# kctl

A CLI tool for making kubernetes commands much easier

## Build

Before building the tool, install `rust` and `cargo`

On Linux and macOS systems, this is done as follows:

```
curl https://sh.rustup.rs -sSf | sh
```

Change namespace from src/main.rs and set your desired namespace

Then run 

```
cargo build --release 
```

You will find the binary in /target folder


## Installation

```
chmod a+x kctl
```

```
mov kctl /usr/local/bin
```



## Kubernetes commands 

1. Getting all pods of your defined namespace.

    ```
    kctl pod
    ```

    Search specific pods with app name

     ```
    kctl pod <app-name>
    ```

2. Getting all services of your defined namespace.
    
    ```
    kctl svc 
    ```

    Search specific service with app name

     ```
    kctl svc <app-name>
    ```

3. Getting all deployments of your defined namespace.

    ```
    kctl deploy 
    ```

    Search specific deploy with app name

     ```
    kctl deploy <app-name>
    ```


4. Checking logs of any of the deployment or app of your defined namespace.

    ```
    kctl logs <app-name>
    ```

    Example: 

    ```
    kctl logs search
    ```

5. Port forward any of the deployment or app of your defined namespace.

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

6. Exec to any pod of the deployment or app of your defined namespace. By default it will try to use `bash`.

     ```
    kctl exec <app-name>
    ```

    Example: 

    ```
    kctl exec search
    ```
