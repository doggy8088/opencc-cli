<p align="center">
  <img src="assets/opencc-cli-readme-banner-web.png" alt="opencc-cli header banner" width="100%">
</p>

# opencc-cli

`opencc-cli` 是一個以 Rust 開發的 Command-Line Interface (CLI) 工具，基於 [`opencc-rust`](https://github.com/doggy8088/opencc-rust) 函式庫，提供跨平台、高效能的中文簡繁體及各地區詞彙轉換功能。

## 功能特點

- **高效能轉換**：基於純 Rust 移植的 `opencc-rust`，保留內嵌字典、Trie 最長匹配與多階段轉換流程。
- **靈活輸入輸出**：支援標準輸入輸出（stdin/stdout）管線操作，或透過參數指定輸入與輸出檔案。
- **支援多種 Locale 預設**：
  - 來源 / 目標 Locale 包括：`cn` (大陸)、`tw` (台灣)、`tw2` (台灣，包含繁體化與常用詞彙轉換)、`twp` (台灣，包含繁體化與大部分詞彙轉換)、`hk` (香港)、`jp` (日本新字體)。
- **自動 Shell 補全**：支援產生常見 Shell（bash、zsh、fish、powershell、elvish）的自動補全指令。

## 安裝方式

### 透過 npm 安裝

```bash
npm install -g @willh/opencc-cli
```

### 從 Release 頁面下載

您可以直接從 [GitHub Releases](https://github.com/doggy8088/opencc-cli/releases) 下載適用於您作業系統（macOS, Linux, Windows）的已編譯二進位檔，解壓後將其加入您的 `PATH` 環境變數中即可。

## 使用說明

### 基礎語法

```bash
opencc-cli [OPTIONS] [COMMAND]
```

### 選項 (Options)

- `-f, --from <LOCALE>`：來源地區/語言設定（預設：`cn`）
- `-t, --to <LOCALE>`：目標地區/語言設定（預設：`tw2`）
- `-i, --input <FILE>`：輸入檔案路徑。若未指定，則從標準輸入（stdin）讀取。
- `-o, --output <FILE>`：輸出檔案路徑。若未指定，則輸出至標準輸出（stdout）。
- `-h, --help`：顯示說明訊息。
- `-V, --version`：顯示版本資訊。

### 支援的 Locale 值

- `cn`：簡體中文
- `tw`：台灣繁體（字形轉換）
- `tw2`：台灣繁體（字形與常用詞彙轉換，例如：公车 -> 公車）
- `twp`：台灣繁體（字形與大部分詞彙轉換，例如：公车 -> 巴士）
- `hk`：香港繁體
- `jp`：日本新字體

---

## 使用範例

### 1. 標準輸入與輸出轉換 (預設 cn -> tw2)

```bash
echo "汉语" | opencc-cli
# 輸出: 漢語

echo "香蕉 公车" | opencc-cli
# 輸出: 香蕉 公車
```

### 2. 指定來源與目標 Locale

將台灣繁體轉換為簡體中文：

```bash
echo "公車與香蕉" | opencc-cli -f tw -t cn
# 輸出: 公车与香蕉
```

將簡體中文轉換為香港繁體：

```bash
echo "公车與香蕉" | opencc-cli -f cn -t hk
# 輸出: 公車與香蕉
```

### 3. 檔案型轉換

讀取 `input.txt` 檔案，將轉換後的內容寫入至 `output.txt`：

```bash
opencc-cli -f cn -t tw2 -i input.txt -o output.txt
```

### 4. 管線串接多個轉換

您可以透過標準輸入/輸出將 `opencc-cli` 與其他命令或自身串接：

```bash
echo "汉语" | opencc-cli -f cn -t tw2 | opencc-cli -f tw2 -t cn
# 輸出: 汉语
```

---

## Shell 自動補全 (Completions)

`opencc-cli` 提供了一個 `completions` 子命令，可用來為您目前使用的 Shell 產生自動補全腳本。

### 支援的 Shell

- `bash`
- `zsh`
- `fish`
- `powershell`
- `elvish`

### 設定方式範例

#### Bash

將補全腳本加入至您的 `.bashrc`：

```bash
opencc-cli completions bash >> ~/.bashrc
```

#### Zsh

將補全腳本寫入至您的 zsh 補全目錄（例如 `/usr/local/share/zsh/site-functions`）：

```bash
opencc-cli completions zsh > /usr/local/share/zsh/site-functions/_opencc-cli
```

---

## 授權條款

本專案採用 **MIT 授權條款**，詳情請參閱 [LICENSE](LICENSE) 檔案。
著作權所有 © 2026 [Will 保哥](https://www.facebook.com/will.fans/)。
