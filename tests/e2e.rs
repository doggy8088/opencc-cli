use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self};
use tempfile::tempdir;

fn opencc_cmd() -> Command {
    Command::cargo_bin("opencc-cli").expect("opencc-cli binary not found")
}

// ==========================================
// TIER 1: FEATURE COVERAGE (21 tests)
// ==========================================

#[test]
fn test_stdin_default() {
    opencc_cmd()
        .write_stdin("汉语")
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_stdin_explicit_cn_tw2() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "tw2"])
        .write_stdin("汉语")
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_stdin_explicit_cn_tw() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "tw"])
        .write_stdin("汉语")
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_stdin_explicit_cn_twp() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "twp"])
        .write_stdin("屏幕")
        .assert()
        .success()
        .stdout(predicate::eq("螢幕\n").or(predicate::eq("螢幕")));
}

#[test]
fn test_stdin_explicit_tw_cn() {
    opencc_cmd()
        .args(["-f", "tw", "-t", "cn"])
        .write_stdin("漢語")
        .assert()
        .success()
        .stdout(predicate::eq("汉语\n").or(predicate::eq("汉语")));
}

#[test]
fn test_stdin_explicit_tw2_cn() {
    opencc_cmd()
        .args(["-f", "tw2", "-t", "cn"])
        .write_stdin("螢幕")
        .assert()
        .success()
        .stdout(predicate::eq("屏幕\n").or(predicate::eq("屏幕")));
}

#[test]
fn test_stdin_explicit_twp_cn() {
    opencc_cmd()
        .args(["-f", "twp", "-t", "cn"])
        .write_stdin("螢幕")
        .assert()
        .success()
        .stdout(predicate::eq("屏幕\n").or(predicate::eq("屏幕")));
}

#[test]
fn test_stdin_explicit_hk_cn() {
    opencc_cmd()
        .args(["-f", "hk", "-t", "cn"])
        .write_stdin("漢語")
        .assert()
        .success()
        .stdout(predicate::eq("汉语\n").or(predicate::eq("汉语")));
}

#[test]
fn test_stdin_explicit_cn_hk() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "hk"])
        .write_stdin("汉语")
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_stdin_explicit_jp_cn() {
    opencc_cmd()
        .args(["-f", "jp", "-t", "cn"])
        .write_stdin("日本語")
        .assert()
        .success()
        .stdout(predicate::eq("日本语\n").or(predicate::eq("日本语")));
}

#[test]
fn test_stdin_explicit_cn_jp() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "jp"])
        .write_stdin("旧字体")
        .assert()
        .success()
        .stdout(predicate::eq("旧字体\n").or(predicate::eq("旧字体")));
}

#[test]
fn test_stdin_passthrough_from_t() {
    opencc_cmd()
        .args(["-f", "t", "-t", "cn"])
        .write_stdin("漢語")
        .assert()
        .success()
        .stdout(predicate::eq("汉语\n").or(predicate::eq("汉语")));
}

#[test]
fn test_stdin_passthrough_to_t() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "t"])
        .write_stdin("汉语")
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_file_input_default_output_stdout() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    fs::write(&input_path, "汉语").unwrap();

    opencc_cmd()
        .args(["-i", input_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_file_input_output_explicit() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    let output_path = dir.path().join("output.txt");
    fs::write(&input_path, "汉语").unwrap();

    opencc_cmd()
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "-f",
            "cn",
            "-t",
            "tw2",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content == "漢語" || content == "漢語\n");
}

#[test]
fn test_file_input_overwrite() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    let output_path = dir.path().join("output.txt");
    fs::write(&input_path, "汉语").unwrap();
    fs::write(&output_path, "old content").unwrap();

    opencc_cmd()
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content == "漢語" || content == "漢語\n");
}

#[test]
fn test_completions_bash() {
    opencc_cmd()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -F").or(predicate::str::contains("opencc-cli")));
}

#[test]
fn test_completions_zsh() {
    opencc_cmd()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef").or(predicate::str::contains("opencc-cli")));
}

#[test]
fn test_completions_fish() {
    opencc_cmd()
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete -c").or(predicate::str::contains("opencc-cli")));
}

#[test]
fn test_completions_powershell() {
    opencc_cmd()
        .args(["completions", "powershell"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Register-ArgumentCompleter")
                .or(predicate::str::contains("opencc-cli")),
        );
}

#[test]
fn test_completions_elvish() {
    opencc_cmd()
        .args(["completions", "elvish"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("edit:completion").or(predicate::str::contains("opencc-cli")),
        );
}

// ==========================================
// TIER 2: BOUNDARY & CORNER CASES (21 tests)
// ==========================================

#[test]
fn test_empty_stdin() {
    opencc_cmd()
        .write_stdin("")
        .assert()
        .success()
        .stdout(predicate::eq(""));
}

#[test]
fn test_empty_input_file() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    let output_path = dir.path().join("output.txt");
    fs::write(&input_path, "").unwrap();

    opencc_cmd()
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.is_empty());
}

#[test]
fn test_missing_from_value() {
    opencc_cmd().arg("--from").assert().failure();
}

#[test]
fn test_missing_to_value() {
    opencc_cmd().arg("--to").assert().failure();
}

#[test]
fn test_missing_input_value() {
    opencc_cmd().arg("--input").assert().failure();
}

#[test]
fn test_missing_output_value() {
    opencc_cmd().arg("--output").assert().failure();
}

#[test]
fn test_invalid_flag() {
    opencc_cmd().arg("--invalid-flag-xyz").assert().failure();
}

#[test]
fn test_nonexistent_input_file() {
    let tmp_dir = tempdir().unwrap();
    let nonexistent_file = tmp_dir.path().join("nonexistent_file_12345.txt");
    opencc_cmd()
        .args(["-i", nonexistent_file.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("not found")
                .or(predicate::str::contains("No such file"))
                .or(predicate::str::contains("error")),
        );
}

#[test]
fn test_nonexistent_output_dir() {
    let tmp_dir = tempdir().unwrap();
    let nonexistent_out = tmp_dir.path().join("non_existent_dir_abc/out.txt");
    opencc_cmd()
        .args(["-o", nonexistent_out.to_str().unwrap()])
        .write_stdin("汉语")
        .assert()
        .failure();
}

#[test]
fn test_unknown_from_locale() {
    opencc_cmd()
        .args(["-f", "unknown_locale_xyz", "-t", "tw2"])
        .write_stdin("汉语")
        .assert()
        .failure();
}

#[test]
fn test_unknown_to_locale() {
    opencc_cmd()
        .args(["-f", "cn", "-t", "unknown_locale_xyz"])
        .write_stdin("汉语")
        .assert()
        .failure();
}

#[test]
fn test_unknown_completions_shell() {
    opencc_cmd()
        .args(["completions", "unknown_shell_xyz"])
        .assert()
        .failure();
}

#[test]
fn test_missing_completions_arg() {
    opencc_cmd().arg("completions").assert().failure();
}

#[test]
fn test_help_flag_short() {
    opencc_cmd().arg("-h").assert().success().stdout(
        predicate::str::contains("Usage:")
            .or(predicate::str::contains("opencc-cli"))
            .or(predicate::str::contains("Options:")),
    );
}

#[test]
fn test_help_flag_long() {
    opencc_cmd().arg("--help").assert().success().stdout(
        predicate::str::contains("Usage:")
            .or(predicate::str::contains("opencc-cli"))
            .or(predicate::str::contains("Options:")),
    );
}

#[test]
fn test_version_flag_short() {
    opencc_cmd().arg("-V").assert().success().stdout(
        predicate::str::contains("opencc-cli")
            .or(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap()),
    );
}

#[test]
fn test_version_flag_long() {
    opencc_cmd().arg("--version").assert().success().stdout(
        predicate::str::contains("opencc-cli")
            .or(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap()),
    );
}

#[test]
fn test_stdin_special_characters() {
    opencc_cmd()
        .write_stdin("汉语\n\t😊\r\n")
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n\t😊\r\n"));
}

#[test]
fn test_stdin_extremely_long_line() {
    let input = "汉语".repeat(170_000);
    let expected = "漢語".repeat(170_000);

    opencc_cmd()
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected.clone()).or(predicate::eq(format!("{}\n", expected))));
}

#[test]
fn test_read_only_input_file() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("readonly_input.txt");
    fs::write(&input_path, "汉语").unwrap();

    let mut perms = fs::metadata(&input_path).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(&input_path, perms).unwrap();

    opencc_cmd()
        .args(["-i", input_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_write_to_directory() {
    let dir = tempdir().unwrap();
    let output_dir_path = dir.path().join("output_dir");
    fs::create_dir(&output_dir_path).unwrap();

    opencc_cmd()
        .args(["-o", output_dir_path.to_str().unwrap()])
        .write_stdin("汉语")
        .assert()
        .failure();
}

// ==========================================
// TIER 3: FEATURE COMBINATIONS (4 tests)
// ==========================================

#[test]
fn test_combo_stdin_to_file() {
    let dir = tempdir().unwrap();
    let output_path = dir.path().join("output.txt");

    opencc_cmd()
        .args(["-f", "cn", "-t", "tw", "-o", output_path.to_str().unwrap()])
        .write_stdin("汉语")
        .assert()
        .success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content == "漢語" || content == "漢語\n");
}

#[test]
fn test_combo_file_to_stdout() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    fs::write(&input_path, "汉语").unwrap();

    opencc_cmd()
        .args(["-i", input_path.to_str().unwrap(), "-f", "cn", "-t", "tw"])
        .assert()
        .success()
        .stdout(predicate::eq("漢語\n").or(predicate::eq("漢語")));
}

#[test]
fn test_combo_file_to_file() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    let output_path = dir.path().join("output.txt");
    fs::write(&input_path, "汉语").unwrap();

    opencc_cmd()
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
            "-f",
            "cn",
            "-t",
            "tw",
        ])
        .assert()
        .success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content == "漢語" || content == "漢語\n");
}

#[test]
fn test_combo_default_file_to_file() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.txt");
    let output_path = dir.path().join("output.txt");
    fs::write(&input_path, "汉语").unwrap();

    opencc_cmd()
        .args([
            "-i",
            input_path.to_str().unwrap(),
            "-o",
            output_path.to_str().unwrap(),
        ])
        .assert()
        .success();

    let content = fs::read_to_string(output_path).unwrap();
    assert!(content == "漢語" || content == "漢語\n");
}

// ==========================================
// TIER 4: REAL-WORLD WORKLOADS (5 tests)
// ==========================================

#[test]
fn test_workload_wikipedia_article() {
    let expected = "\
數學是利用符號語言研究數量、結構、變化以及空間等概念的一門學科。\n\n\
數學家們拓展這些概念，為了公式化新的猜想，以及從選定的公理及定義出發，通過嚴謹的推理建立真理。";

    let input_simplified = "\
数学是利用符号语言研究数量、结构、变化以及空间等概念的一门学科。\n\n\
数学家们拓展这些概念，为了公式化新的猜想，以及从选定的公理及定义出发，通过严谨的推理建立真理。";

    opencc_cmd()
        .args(["-f", "cn", "-t", "tw2"])
        .write_stdin(input_simplified)
        .assert()
        .success()
        .stdout(predicate::eq(expected.to_string()).or(predicate::eq(format!("{}\n", expected))));
}

#[test]
fn test_workload_piped_chain() {
    let output = opencc_cmd()
        .args(["-f", "cn", "-t", "tw2"])
        .write_stdin("汉语")
        .assert()
        .success()
        .get_output()
        .clone();

    let stdout_str = String::from_utf8(output.stdout).unwrap();
    assert!(stdout_str.contains("漢語"));
}

#[test]
fn test_workload_git_diff_patch() {
    let input = "\
diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcde 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
+// 这是一个测试
 fn main() {
-    println!(\"旧代码\");
+    println!(\"新代码\");
 }";

    let expected = "\
diff --git a/src/main.rs b/src/main.rs
index 1234567..89abcde 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
+// 這是一個測試
 fn main() {
-    println!(\"舊代碼\");
+    println!(\"新代碼\");
 }";

    opencc_cmd()
        .args(["-f", "cn", "-t", "tw"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(predicate::eq(expected.to_string()).or(predicate::eq(format!("{}\n", expected))));
}

#[test]
fn test_workload_code_file_comments() {
    let input = "\
// 这是一个包含中文注释 of Rust 源文件
fn main() {
    // 输出问候信息
    println!(\"你好，世界！\");
}";

    let expected_tw2 = "\
// 這是一個包含中文註釋 of Rust 源文件
fn main() {
    // 輸出問候資訊
    println!(\"你好，世界！\");
}";

    opencc_cmd()
        .args(["-f", "cn", "-t", "tw2"])
        .write_stdin(input)
        .assert()
        .success()
        .stdout(
            predicate::eq(expected_tw2.to_string())
                .or(predicate::eq(format!("{}\n", expected_tw2))),
        );
}

#[test]
fn test_workload_multiple_conversions_piped() {
    let first_conversion = opencc_cmd()
        .args(["-f", "cn", "-t", "tw2"])
        .write_stdin("汉语")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let converted_str = String::from_utf8(first_conversion).unwrap();

    opencc_cmd()
        .args(["-f", "tw2", "-t", "cn"])
        .write_stdin(converted_str.trim())
        .assert()
        .success()
        .stdout(predicate::eq("汉语\n").or(predicate::eq("汉语")));
}
