use crate::common::{cmd_snapshot, TestContext};

/// GitHub Action only has docker for linux hosted runners.
#[test]
fn docker() {
    let context = TestContext::new();
    context.init_project();

    context.write_pre_commit_config(indoc::indoc! {r#"
        repos:
          - repo: https://github.com/j178/pre-commit-docker-hooks
            rev: master
            hooks:
              - id: hello-world
                entry: "echo Hello, world!"
                verbose: true
                always_run: true
    "#});

    context.git_add(".");

    cmd_snapshot!(context.filters(), context.run(), @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    Cloning https://github.com/j178/pre-commit-docker-hooks@master
    Installing environment for https://github.com/j178/pre-commit-docker-hooks@master
    Hello World..............................................................Passed
    - hook id: hello-world
    - duration: [TIME]
      Hello, world! .pre-commit-config.yaml

    ----- stderr -----
    "#);
}
