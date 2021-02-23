## Background

- Do you use conventional commits in your daily workflow?
- Do you specify the scope(s) every time for the packages you're working on?

If so this project might save you some time. It provides you with a [git prepare-commit-msg hook](https://git-scm.com/docs/githooks#_prepare_commit_msg) that figures out the automatable parts of the commit for you.

### Installation

Download the prebuilt binary from the [releases page](https://github.com/danielcondemarin/monorepo-conventional-commits/releases)

Then simply add the prepare-commit-msg binary to the repo git hooks. 

```
  $ cp ~/Downloads/prepare-commit-msg /your_monorepo/.git/hooks
```

Currently only macos is available from the releases page. For other platforms you could build from source using [rust cargo](https://github.com/rust-lang/cargo).

### Usage

- Running `git commit` will automatically run `prepare-commit-msg` for you. The commit editor should look like this:

```
chore(app1):

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
...
```

The first line of the commit message will be pre-populated for you. You could at this point change the commit type and finish the rest of the commit message.

- You could also do `git commit -m "docs: my commit message"`.

In this case git doesn't invoke the editor, but the precommit hook will add for you the scope if there is one, so the resulting commit could be something like: `docs(app1): my commit message`

Any of the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) types should work.

Other commit sources such as merge, squash and commit are ignored.

### Supported monorepos

- [Lerna](https://github.com/lerna/lerna)

### Roadmap

- Look at implementing support for other monorepos in https://github.com/korfuri/awesome-monorepo

### Other resources

- [prepare-commit-msg docs](https://git-scm.com/docs/githooks#_prepare_commit_msg)
- [configuring global git hooks](https://coderwall.com/p/jp7d5q/create-a-global-git-commit-hook)
