# GitHub Actions (Safely)

## Disclaimer
I do not condone trying anything I am about to show you.

## What is it?

It allows you to run code in a way that easily linked to the lifecycle of the code you're working with. Doing common tasks like running unit tests, linting code, deploying code to different environments, etc. 

## Who is using this?

At UF Applications, Data, and Solutions integrate GitHub Actions into their daily routines, either directly or indirectly. This tool has also become a big part of modern software development practices industry wide. 

## Where is the code?

I wrote a simple rust application that needs to be tested and deployed, shout out to Raphael for the Rust inspiration.

```yaml
name: Rust
```

This is the name of the workflow. It's what you'll see on GitHub when the action is running.
on:


```yaml
on:
  push:
  pull_request:
    branches: [ main ]
```
This defines the events that will trigger the workflow. 
In this case, there are two events:
- `push:` The workflow will run when code is pushed to any branch. 
- `pull_request`: The workflow will also run when a pull request is made to the main branch.

```yaml
jobs:
    build:
```

Jobs are a set of steps that execute on the same runner. This workflow contains a single job called build.


```yaml
jobs:
  build:
    runs-on: ubuntu-latest
```

This specifies the type of OS to run the job on. In this case, it's the latest version of Ubuntu (Linux distro), and what we generally use at UF.

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
```

Steps are individual tasks that run commands in the job, think of them as a string of bash commands you can run on one machine. The build job consists of the following steps:

```yaml
- uses: actions/checkout@v3
```
This step checks out your repository code so the other steps can access it, it is a pre-built service maintained by GitHub.

```yaml
- name: Install Rust
    run: |
    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSfy | sh
    source $HOME/.cargo/env
```
- A named step to install Rust for other steps to use. Since we are on Ubuntu and are sure other steps will be on the same "runner" we can install Rust here for use later on. `run` just means arbtrarily run any bash command you want, with the `|` allowing you to span multiple lines.

```yaml
- name: Build
    run: cargo build --verbose
    working-directory: ./demo
```
Rust is a compiled language, this step does the building. Of note is the `working-directory: ./demo`, this is just an easy way to say run this step's `run` in a different directory than the default. 

```yaml
- name: Run tests
    run: cargo test --verbose
    working-directory: ./demo
```
This actually runs the tests, if any fail there will be a big red X in the GitHub UI, otherwise it will be a sweet sweet green check!


Checkout the file at [lint.yaml](./.github/workflows/lint.yaml)

