use assert_fs::fixture::{FileWriteStr, PathChild};

use crate::common::{cmd_snapshot, TestContext};

mod common;

#[test]
fn validate_config() -> anyhow::Result<()> {
    let context = TestContext::new();

    // No files to validate.
    cmd_snapshot!(context.filters(), context.validate_config(), @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    "#);

    context
        .workdir()
        .child(".pre-commit-config.yaml")
        .write_str(indoc::indoc! {r"
            repos:
              - repo: https://github.com/pre-commit/pre-commit-hooks
                rev: v5.0.0
                hooks:
                  - id: trailing-whitespace
                  - id: end-of-file-fixer
                  - id: check-json
        "})?;
    // Validate one file.
    cmd_snapshot!(context.filters(), context.validate_config().arg(".pre-commit-config.yaml"), @r#"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    "#);

    context
        .workdir()
        .child("config-1.yaml")
        .write_str(indoc::indoc! {r"
            repos:
              - repo: https://github.com/pre-commit/pre-commit-hooks
        "})?;

    // Validate multiple files.
    cmd_snapshot!(context.filters(), context.validate_config().arg(".pre-commit-config.yaml").arg("config-1.yaml"), @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    error: Failed to parse `config-1.yaml`
      caused by: repos: Invalid remote repo: missing field `rev` at line 2 column 3
    "#);

    Ok(())
}
