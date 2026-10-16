//! Help text for `-h` (quick reference) and `--help` (full guide, written so an
//! AI agent can use the tool correctly from this output alone).

use clap::builder::styling::{Style, Styles};

const STYLES: Styles = Styles::styled();
const HEADER: Style = *STYLES.get_header();
const LITERAL: Style = *STYLES.get_literal();

pub const ABOUT: &str =
    "Convert Chinese text between Simplified, Traditional, and regional variants (OpenCC).";

pub const LONG_ABOUT: &str = "\
Convert Chinese text between Simplified, Traditional, and regional variants (OpenCC).

Converts both character forms (汉 → 漢) and region-specific vocabulary (软件 → 軟體).
Offline and dictionary-based: no network, no AI model, same input → same output.
It does NOT translate between languages and does NOT detect the input locale.";

pub const USAGE: &str = "\
opencc-cli [OPTIONS] < INPUT > OUTPUT
       opencc-cli [OPTIONS] -i <FILE> -o <FILE>
       opencc-cli completions <SHELL>";

fn heading(title: &str) -> String {
    format!("{HEADER}{title}:{HEADER:#}")
}

fn literal(text: &str) -> String {
    format!("{LITERAL}{text}{LITERAL:#}")
}

/// Shown after the options in `-h`.
pub fn after_help() -> String {
    format!(
        "\
{locales}
  {tw2}  Taiwan Traditional + Taiwan phrases  ★ recommended, default --to
  {twp}  Taiwan Traditional + broader Taiwan vocabulary
  {tw}   Taiwan Traditional, character forms only
  {hk}   Hong Kong Traditional
  {cn}   Simplified Chinese                   default --from
  {jp}   Japanese Shinjitai (新字体)
  {t}    OpenCC standard Traditional (no regional rules)

{examples}
  echo '软件和鼠标' | opencc-cli                 # 軟體和滑鼠
  echo '軟體和滑鼠' | opencc-cli -f tw2 -t cn    # 软件和鼠标
  opencc-cli -i in.txt -o out.txt

Run 'opencc-cli --help' for the full guide (locale details, behavior, exit codes).",
        locales = heading("Locales"),
        examples = heading("Examples"),
        tw2 = literal("tw2"),
        twp = literal("twp"),
        tw = literal("tw"),
        hk = literal("hk"),
        cn = literal("cn"),
        jp = literal("jp"),
        t = literal("t"),
    )
}

/// Shown after the options in `--help`.
pub fn after_long_help() -> String {
    format!(
        "\
{locales}
  {cn}   Simplified Chinese (Mainland China). Default --from.
  {tw2}  Taiwan Traditional: characters + common Taiwan phrases. Default --to.
       ★ Recommended for any Taiwan output: best overall conversion quality.
       Uses 台 in everyday words (台北, 台灣) and keeps 臺 in official names
       (國立臺灣大學, 新臺幣).
  {twp}  Taiwan Traditional: characters + the broader upstream OpenCC Taiwan
       phrase set (出租车 → 計程車). Uses 臺 everywhere (臺北).
  {tw}   Taiwan Traditional: character forms only; vocabulary is unchanged.
  {hk}   Hong Kong Traditional character forms.
  {jp}   Japanese Shinjitai (新字体) character forms.
  {t}    OpenCC standard Traditional with no regional rules (see below).

  Same input with each --to (input: 软件 鼠标 出租车 信息 头发 台北 新台币):
    -t tw2  軟體 滑鼠 出租車 資訊 頭髮 台北 新臺幣
    -t twp  軟體 滑鼠 計程車 資訊 頭髮 臺北 新臺幣
    -t tw   軟件 鼠標 出租車 信息 頭髮 臺北 新臺幣
    -t hk   軟件 鼠標 出租車 信息 頭髮 台北 新台幣
    -t jp   軟件 鼠標 出租車 信息 頭髪 台北 新台幣
    -t t    軟件 鼠標 出租車 信息 頭髮 臺北 新臺幣

{choosing}
  Simplified → Taiwan               -f cn  -t tw2   (default; recommended)
  Taiwan → Simplified               -f tw2 -t cn
  Hong Kong → Taiwan                -f hk  -t tw2
  Taiwan → Hong Kong                -f tw2 -t hk
  Normalize existing Taiwan text    -f tw2 -t tw2   (fixes 台/臺 and phrases)
  Unknown Traditional → Taiwan      -f t   -t tw2
  Set --from to how the input is actually written; a wrong --from silently
  produces poor output instead of an error.

{how}
  Every conversion runs in two stages through standard Traditional (t):
      input ──[--from dictionaries]──▶ t ──[--to dictionaries]──▶ output
  Any --from/--to pair is valid. Using t on one side skips that stage;
  -f t -t t copies input unchanged. Conversion is one-to-many, so a round
  trip (cn → tw2 → cn) is not guaranteed to reproduce the original text.

{examples}
  $ echo '汉语软件' | opencc-cli
  漢語軟體

  $ echo '国立台湾大学在台北' | opencc-cli -f cn -t tw2
  國立臺灣大學在台北

  $ echo '軟體 滑鼠' | opencc-cli -f tw2 -t cn
  软件 鼠标

  $ opencc-cli -f cn -t tw2 -i README.md -o README.zh-TW.md

  # Convert a file in place (write to a temp file, then replace)
  $ opencc-cli -i doc.md -o doc.md.tmp && mv doc.md.tmp doc.md

  # Pass multi-line text without shell quoting issues
  $ opencc-cli <<'EOF'
  第一行文字
  第二行文字
  EOF

{behavior}
  - Input and output are UTF-8. Invalid UTF-8 input fails with exit status 1.
  - Text is streamed line by line, so large files and pipes are fine.
  - Only Chinese characters change. ASCII, code, Markdown, punctuation, and
    line endings (LF or CRLF) pass through unchanged.
  - stdout carries only converted text; errors go to stderr as 'error: ...'.
  - -o creates the file or overwrites it without asking.
  - Never pass the same path to -i and -o: the file is truncated before it is
    read and ends up empty. Use a temp file as shown above.

{exit}
  0  Success
  1  Runtime error (cannot read input, cannot write output, invalid UTF-8)
  2  Usage error (unknown option, invalid locale, missing value)",
        locales = heading("Locales"),
        choosing = heading("Choosing --from / --to"),
        how = heading("How conversion works"),
        examples = heading("Examples"),
        behavior = heading("Behavior"),
        exit = heading("Exit status"),
        cn = literal("cn"),
        tw2 = literal("tw2"),
        twp = literal("twp"),
        tw = literal("tw"),
        hk = literal("hk"),
        jp = literal("jp"),
        t = literal("t"),
    )
}
