# clonymccloneface
Clone all your github repos AND set upstream for your forks

## Demo

You need to:

- have git installed
- have a personal access token

```shell script
$ clonymccloneface --token "df79XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX" -u cjrh .
✅ Cloned dockerctx
✅ Cloned easycython
✅ Cloned enumerate_reversible
✅ Cloned excitertools
✅ Cloned fileinput.rs  (and set upstream repo)
⡏ Cloning flashtext...

```

## Overview

For each repo in your github account, it will:

- Clone the repo
- If the repo is a fork, it will also set the "upstream" remote in your 
  working copy.

Roughly, the equivalent of these commands:

```
$ git clone git@github.com:<user>/<repo>.git
$ cd <repo>
$ git remote add upstream git@github.com:<upstream>/<repo>.git
```
