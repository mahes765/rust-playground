# Git Rewang

<p align="center">
  <img src="assets/gr-lg.png" alt="Preview"/>
</p>

Git Rewang is a lightweight CLI tool for developers who use multiple GitHub accounts. It streamlines commit, push, fetch, and pull operations with different identities, reducing the risk of committing with the wrong account.

# Structure project

```
git-helper/
│
├─ src/
│  ├─ main.rs
│  ├─ cli.rs
│  ├─ git/
│  │   ├─ mod.rs
│  │   ├─ repo.rs
│  │   ├─ branch.rs
│  │   ├─ commit.rs
│  │   └─ status.rs
│  │
│  ├─ template/
│  │   ├─ mod.rs
│  │   └─ loader.rs
│  │
│  ├─ config/
│  │   ├─ mod.rs
│  │   └─ config.rs
│  │
│  └─ utils/
│      └─ fs.rs
│
├─ git-helper.toml
└─ Cargo.toml
```

---

# Alur program (high-level)

```
CLI input
   ↓
Parse command
   ↓
Load config
   ↓
Detect git repo
   ↓
Execute feature
   ↓
Print result
```

---

# CLI command design

Contoh:

```
git-helper commit
git-helper commit -t feat
git-helper clean-branches
git-helper status
```

CLI parser cocok:

* clap

---

# Modul Git core

Gunakan binding Git native:

* git2

Kenapa git2:

* tidak perlu shell git
* cepat
* cross-platform
* API lengkap

---

# 1) Auto commit template

Tujuan: commit message konsisten.

Contoh template:

```
feat: {message}

branch: {branch}
date: {date}
```

## Alur logika commit

```
open repo
  ↓
cek staged files
  ↓
deteksi branch aktif
  ↓
load template
  ↓
replace placeholder
  ↓
create commit
```

## Pseudocode

```
repo = open_repo()
index = repo.index()

if index.is_empty():
    print("No staged files")

branch = repo.current_branch()

template = load_template(type)

message = fill_template(template, branch, date)

create_commit(repo, message)
```

---

# 2) Branch cleaner

Tujuan: hapus branch lokal yang sudah merged.

## Alur logika

```
open repo
  ↓
get all local branches
  ↓
get current branch
  ↓
check merged to main
  ↓
delete merged branch
```

## Pseudocode

```
branches = repo.branches(local)

for b in branches:
    if b != current && merged_to_main(b):
        delete(b)
```

---

# 3) Status ringkas

Lebih simpel dari `git status`.

Output:

```
branch: feature/login
staged: 3 files
modified: 2 files
untracked: 1 file
```

## Alur

```
open repo
get branch
get status entries
count types
print summary
```

git2 punya API status langsung.

---

# Modul Template

Folder: `template/loader.rs`

Template bisa:

* global
* per project

Lokasi:

```
~/.config/git-helper/templates/
project/.git-helper/
```

Loader:

```
if project template exists:
    load
else:
    load global
```

---

# Modul Config

File: `git-helper.toml`

Contoh:

```
default_branch = "main"

[commit_types]
feat = "✨ feat: {message}"
fix = "🐛 fix: {message}"
docs = "📝 docs: {message}"
```

Library:

* serde
* toml

---

# Struktur eksekusi main.rs

Flow utama:

```
parse cli
match command:
   commit -> git::commit::run()
   clean  -> git::branch::clean()
   status -> git::status::show()
```

---

# Error handling

Rust idiomatic:

```
Result<T, GitHelperError>
```

Error type:

```
enum GitHelperError {
    NotRepo,
    NoStagedFiles,
    Git(git2::Error),
    Io(std::io::Error)
}
```

---

# Library tambahan opsional

Filesystem:

* dirs

Tanggal:

* chrono

CLI warna:

* colored

---

# Contoh alur nyata (commit)

User:

```
git-helper commit -t feat "add login"
```

Flow:

```
CLI parse → type=feat
↓
load config
↓
open repo
↓
current branch = feature/auth
↓
load template feat
↓
message = "✨ feat: add login"
↓
create commit
↓
done
```