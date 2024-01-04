# GitHub Actions (Safely)

## Disclaimer
I do not condone trying anything I am about demo.

## What are Github Actions?

They allows you to run code in a way that easily links to the git lifecycle of the code you're working with. For common tasks like running unit tests, linting code, deploying code to different environments, etc. All during different git/github events, like commits to a certain branch or when opening a pull request.

## Who is using this?

At UF Applications, Data, and Solutions generally use GitHub Actions daily, directly or indirectly. This tool has also become a big part of modern software development practices industry wide. 

## What is this .github folder?

That is where github actions live, in `.github/workflows`. Any yaml file in there is understood to be a special file for configuring with GitHub Action workflows. Let's look more closely at one below.

```yaml
name: Deploy to S3
```

This is the name of the workflow. It's what you'll see in GitHub when the action is running.
```yaml
on:
  push:
    branches: [ main ]
```
This defines the events that will trigger the workflow. 
In this case, there are two events:
- `push:` The workflow will run when code is pushed to any branch. 
```yaml
jobs:
    deploy:
```

Jobs are a set of steps that execute on the same runner (a runner is like a VM). This workflow contains a single job called deploy.


```yaml
jobs:
  deploy:
    runs-on: ubuntu-latest
```

This specifies what to run the job on. In this case, it's the latest version of Ubuntu a Linux distro, and what we generally use at UF.

```yaml
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
```

Steps are individual tasks that run commands in the job, think of them as literally a list of steps to do in order. 

```yaml
- uses: actions/checkout@v4
```
This step checks out the repository code so the other steps can access it. It is a pre-built step that is being imported, maintained by third party. This pre-built logic used in the step is called an action. 

```yaml
- name: Set up AWS CLI
  uses: aws-actions/configure-aws-credentials@v4
  with:
    aws-access-key-id: ${{ secrets.AWS_ACCESS_KEY }}
    aws-secret-access-key: ${{ secrets.AWS_SECRET_KEY }}
    aws-region: us-east-1
```
Another action imported that is maintained by a third party. This one installs and logs the following steps into the aws cli tool. The `with` keyword lets you pass variables to the step, in this case we pass the the credentials needed to login to the aws cli.

`${{ secrets.AWS_SECRET_KEY }}` is GitHub syntax to access the secrets stored in the repository. This is so we don't have secrets visible to anyone who can see the code. This is where the problems start to arise. 

```yaml
- name: Deploy to S3
  run: aws s3 cp index.html s3://uf-gha-demo/index.html --acl public-read
```
This uses the aws cli, installed in the previous step, to upload the `index.html` file installed in the first step, to s3 (a place to host websites and store stuff)!


Checkout the file at [deploy-html.yaml](./.github/workflows/deploy-html.yaml)

## What is the issue?

Unfortunately, storing secrets, even using the secrets syntax, is essentially giving the secret to anyone with access to GitHub without the proper precautions. Something we have not done in every repository as of yet. 

People can change the action to run not on a certain branch, but on their feature branch in a pull request. Accessing all secrets without limits in the action. They would also have a logged in aws cli at their fingertips, giving often admin access to all of aws (including Google Cloud Platform in some cases). 

This means a malicious actor, or an unknowing intern, could wreck permanent havock to all of UF. All they need is the ability to create a pull request in our repos. 

## Uhh how can we fix that?

The solution we came to are GitHub Environments! They are a way to group repo secrets so they can only be accessed under certain defined situations. This means remove all global repo secrets, and put those secrets in a `prod` environment that only runs on `main` which is a protected branch, for example.

Note: This is still an issue with Identity Federation (which is a big problem), but I believe would be possible to prevent it by using assertions based on the branch. 