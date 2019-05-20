# food

A CLI tool for making developer life easy. 

## Kubernetes commands 

1. Getting all pods in `food` namepaces

    ```
    food pod
    ```
2. Getting all services in `food` namespaces
    
    ```
    food svc 
    ```

3. Getting all deployments in `food` namespaces

    ```
    food deploy 
    ```


4. Checking logs of any of the `food` deployment or app

    ```
    food logs <app-name>
    ```

    Example: 

    ```
    food logs search
    ```

5. Port forward any of the `food` deployment or app

    ```
    food forward <app-name> <port-number>
    ```

    Example: 

    ```
    food forward search 8030
    ```

    or 

    ```
    food forward search 8030:8030
    ```

6. Exec to any pod of the `food` deployment or app. By default it will try to use `bash`.

     ```
    food exec <app-name>
    ```

    Example: 

    ```
    food exec search
    ```