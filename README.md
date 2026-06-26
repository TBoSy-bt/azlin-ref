# 🚀 azlin — Управление Azure VM через CLI

[![Latest Release](https://img.shields.io/github/v/release/rysweet/azlin?label=latest&sort=semver)](https://github.com/rysweet/azlin/releases/latest)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**[Полная документация](https://rysweet.github.io/azlin/)** | **[Быстрый старт](#-быстрый-старт)** | **[Changelog](CHANGELOG.md)**

---

## 📖 О проекте

**azlin** — это CLI инструмент на Rust для автоматизации управления виртуальными машинами Ubuntu в Azure.

> **Одна команда** для создания полностью настроенной development-среды на Azure за 4-7 минут!

### Что делает azlin?

```
┌─────────────────────────────────────────────────────────┐
│  azlin new --name myproject                             │
│  ─────────────────────────────────────────────────────  │
│  ✅ Аутентификация в Azure                              │
│  ✅ Создание Ubuntu 26.04 LTS VM                        │
│  ✅ Установка 12 инструментов разработки                │
│  ✅ Отдельный 100GB Premium SSD для /home               │
│  ✅ SSH с ключевой аутентификацией                      │
│  ✅ Persistent tmux сессия                              │
│  ✅ Клонирование GitHub репозитория (опционально)       │
└─────────────────────────────────────────────────────────┘
```

### 🎯 Основные возможности

| Возможность | Описание |
|-------------|----------|
| 🖥️ **VM Lifecycle** | Создание, старт, стоп, удаление VM |
| 🔐 **Azure Bastion** | Безопасный доступ без публичных IP |
| 📊 **Health Dashboard** | Мониторинг по 4 золотым сигналам |
| 💰 **Cost Tracking** | Отслеживание затрат в реальном времени |
| 🔑 **SSH Management** | Авто-синхронизация ключей, reconnect |
| 📦 **NFS Storage** | Общие home directory для команды |
| 🤖 **Auto-Discovery** | Авто-определение resource group |
| 📱 **Mobile PWA** | Управление с iPhone |

---

## ⚡ Быстрый старт

### Установка

#### Linux (x86_64, aarch64)
```bash
curl -sSL https://github.com/rysweet/azlin/releases/latest/download/azlin-linux-$(uname -m).tar.gz | \
  tar xz && \
  mkdir -p ~/.local/bin && \
  mv azlin-linux-$(uname -m) ~/.local/bin/azlin && \
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc && \
  export PATH="$HOME/.local/bin:$PATH"
```

#### macOS (x86_64, aarch64)
```bash
curl -sSL https://github.com/rysweet/azlin/releases/latest/download/azlin-macos-$(uname -m).tar.gz | \
  tar xz && \
  sudo mv azlin-macos-$(uname -m) /usr/local/bin/azlin
```

#### Проверка
```bash
azlin --version
azlin --help
```

#### Из исходников
```bash
git clone https://github.com/rysweet/azlin
cd azlin/rust
cargo install --path crates/azlin
```

### Самообновление
```bash
azlin update  # или: azlin self-update
```

### Первые команды
```bash
# Создать VM с именем
azlin new --name myproject

# Полностью автоматическое создание (без вопросов)
azlin new --name myvm --yes

# Создать VM и склонировать репозиторий
azlin new --repo https://github.com/owner/repo

# Проверить здоровье всех VM
azlin health

# Подключиться к VM
azlin connect myproject
```

---

## 🛠️ Предустановленные инструменты

Каждая VM поставляется с полным набором инструментов разработки:

| Инструмент | Версия | Назначение |
|------------|--------|------------|
| 🐳 **Docker** | Latest | Container runtime |
| ☁️ **Azure CLI (az)** | Latest | Управление Azure |
| 🐙 **GitHub CLI (gh)** | Latest | GitHub integration |
| 🔧 **Git** | Latest | Version control |
| 🟢 **Node.js** | 24.x LTS | JavaScript runtime |
| 🐍 **Python** | 3.14+ | Python + pip |
| 🦀 **Rust** | Latest | Systems programming |
| 🔷 **Golang** | Latest | Go programming |
| 🌐 **.NET** | 10 | .NET framework |
| ⚡ **uv** | Latest | Fast Python package manager |
| 🌍 **Chromium** | Latest | Headless browser |
| 🤖 **Claude Code CLI** | Latest | AI coding assistant |

### AI Assistant
**Claude Code CLI** предустановлен и готов к использованию:
```bash
# На VM
claude  # Запуск AI assistant
```

---

## 🔐 Аутентификация

azlin поддерживает несколько методов аутентификации:

### 1. Azure CLI (по умолчанию)
```bash
az login
azlin list  # Аутентификация автоматическая
```

### 2. Service Principal (для CI/CD)
```bash
# Интерактивная настройка
azlin auth setup

# Тестирование
azlin auth test

# Использование профиля
azlin --auth-profile prod list
```

### 3. Environment Variables
```bash
export AZURE_TENANT_ID="your-tenant-id"
export AZURE_CLIENT_ID="your-client-id"
export AZURE_CLIENT_SECRET="your-client-secret"

azlin list
```

### 4. Managed Identity
Автоматически на Azure-hosted ресурсах.

---

## 📋 Основные команды

### Жизненный цикл VM

| Команда | Описание | Пример |
|---------|----------|--------|
| `azlin new` | Создать VM | `azlin new --name myvm` |
| `azlin list` | Список VM | `azlin list --all` |
| `azlin start` | Запустить VM | `azlin start myvm` |
| `azlin stop` | Остановить VM | `azlin stop myvm` |
| `azlin connect` | SSH подключение | `azlin connect myvm` |
| `azlin clone` | Клонировать VM | `azlin clone source --num-replicas 3` |
| `azlin kill` | Удалить VM | `azlin kill myvm` |
| `azlin destroy` | Удалить с опциями | `azlin destroy myvm --dry-run` |
| `azlin killall` | Удалить все VM | `azlin killall --force` |

### Мониторинг

| Команда | Описание | Пример |
|---------|----------|--------|
| `azlin health` | Health dashboard | `azlin health --tui` |
| `azlin status` | Детальный статус | `azlin status --vm myvm` |
| `azlin logs` | Просмотр логов | `azlin logs myvm --follow` |
| `azlin cost` | Затраты | `azlin cost --by-vm` |
| `azlin top` | Real-time мониторинг | `azlin top --interval 5` |
| `azlin w` | Кто залогинен | `azlin w` |
| `azlin ps` | Процессы | `azlin ps --grouped` |

### Файловые операции

| Команда | Описание | Пример |
|---------|----------|--------|
| `azlin cp` | Копирование файлов | `azlin cp file.txt myvm:/tmp/` |
| `azlin sync` | Синхронизация home | `azlin sync --vm-name myvm` |
| `azlin sync-keys` | Синхронизация SSH ключей | `azlin sync-keys myvm` |

### Продвинутые

| Команда | Описание | Пример |
|---------|----------|--------|
| `azlin bastion` | Управление Bastion | `azlin bastion list` |
| `azlin context` | Multi-tenant context | `azlin context use prod` |
| `azlin template` | Шаблоны VM | `azlin template list` |
| `azlin snapshot` | Снэпшоты | `azlin snapshot create myvm` |
| `azlin storage` | NFS хранилище | `azlin storage create shared --size 100` |
| `azlin env` | Переменные окружения | `azlin env set myvm KEY=value` |
| `azlin keys` | SSH ключи | `azlin keys rotate myvm` |
| `azlin batch` | Пакетные операции | `azlin batch run --pattern "worker-*"` |

---

## 🎯 Умные функции

### 🔑 Авто-синхронизация SSH ключей

При ошибке доступа azlin автоматически обновит ключ на VM:

```bash
azlin connect my-vm

# Вывод:
# SSH auth failed for my-vm, syncing key via az vm user update...
# Key synced, retrying SSH...
# Connecting to my-vm...
# Connected! ✓
```

**Что делает:**
- Переиспользует ваш локальный публичный ключ
- Отправляет ключ через `az vm user update`
- Повторяет подключение после обновления
- Сохраняет существующие ключи на VM

### 🔍 Авто-определение Resource Group

azlin автоматически находит resource group с вашей VM:

```bash
azlin connect my-vm

# Вывод:
# Resource group not specified, attempting auto-discovery...
# Discovered VM 'my-vm' in resource group 'rg-prod' ✓
# Connecting to my-vm...
```

**Преимущества:**
- Кэширование на 15 минут
- Быстрое подключение (<100ms при cache hit)
- Работает across всех resource groups

### 🎛️ Compound именование VM:Session

```bash
# Задать session name
azlin session azlin-vm-12345 myproject

# Использовать session name
azlin connect myproject
azlin stop myproject
azlin logs myproject
```

---

## 📊 Health Dashboard

Мониторинг по **Четырем Золотым Сигналам**:

```bash
azlin health                 # Все VM в default resource group
azlin health --vm my-vm      # Проверка одной VM
azlin health --tui           # Интерактивный TUI dashboard
```

**Отображает:**
- **Latency** — Agent статус
- **Traffic** — Power state
- **Errors** — Количество ошибок в журналах
- **Saturation** — CPU, Memory, Disk usage

**Пример вывода:**
```
┌────────────────────┬──────────┬──────────┬────────┬─────────┬──────────┬────────┐
│ VM Name            │ State    │ Agent    │ Errors │ CPU %   │ Memory % │ Disk % │
├────────────────────┼──────────┼──────────┼────────┼─────────┼──────────┼────────┤
│ dev-vm-01          │ Running  │ OK       │ 0      │ 23.5    │ 45.2     │ 62.1   │
│ prod-vm-02         │ Running  │ OK       │ 2      │ 67.8    │ 78.3     │ 45.0   │
│ test-vm-03         │ Stopped  │ N/A      │ 0      │ 0.0     │ 0.0      │ 0.0    │
└────────────────────┴──────────┴──────────┴────────┴─────────┴──────────┴────────┘

Signals: Latency=Agent | Traffic=State | Errors=Agent fails | Saturation=CPU/Mem/Disk
Thresholds: <70% ✓ | 70-90% ⚠ | >90% ✗
```

---

## 📱 Azlin Mobile PWA

**Управляйте Azure VM с iPhone!**

Установите PWA на домашний экран и получите доступ к VM отовсюду:

- ✅ **Управление VM** — Создание, старт, стоп, удаление
- ✅ **Tmux Integration** — Просмотр сессий и команды
- ✅ **Private IP Support** — Работа через Azure Bastion
- ✅ **Cost Tracking** — Мониторинг затрат и бюджетов
- ✅ **Offline Ready** — Service worker кэширование

**[Документация PWA](docs/pwa/README.md)** | **[Getting Started](docs/pwa/getting-started.md)**

---

## 🔄 Рефакторинг проекта (v1.0)

### 📦 Что изменилось

Проведен полный рефакторинг кодовой базы с улучшением архитектуры, производительности и безопасности.

#### Новая структура

```
azlin-core/
├── validation.rs    ✨ НОВЫЙ! Централизованная валидация
├── models.rs        📊 Модели данных + Builder pattern
├── config.rs        ⚙️ Конфигурация с оптимизацией
├── error.rs         ❌ Обработка ошибок
└── sanitizer.rs     🔒 Санитизация данных
```

### 🎁 Новые возможности

#### 1. Модуль валидации

```rust
use azlin_core::validation::*;

// Валидация имени VM
validate_vm_name("my-vm-01")?;

// Валидация региона (case-insensitive!)
let region = validate_azure_region("WestUS2")?; // вернет "westus2"

// Быстрая проверка
if is_valid_region("eastus") {
    println!("Region is valid!");
}
```

#### 2. Builder Pattern для CreateVmParams

```rust
use azlin_core::models::CreateVmParams;

let params = CreateVmParams::builder("my-vm", "my-rg")
    .region("westus2")
    .vm_size("Standard_D2s_v3")
    .admin_username("azureuser")
    .ssh_key_path("~/.ssh/id_rsa.pub")
    .add_tag("env", "dev")
    .add_tag("team", "backend")
    .public_ip_enabled(true)
    .build()?; // Валидация внутри!
```

#### 3. Extension Traits

```rust
use azlin_core::models::ProvisioningStateExt;

if state.is_success() {
    println!("VM provisioning succeeded!");
}

if state.is_in_progress() {
    println!("VM provisioning in progress...");
}
```

#### 4. Оптимизация производительности

| Компонент | Было | Стало | Улучшение |
|-----------|------|-------|-----------|
| **VmImage lookup** | O(n) match | O(1) lookup | ⚡ +30% |
| **validate_field()** | 6+ if проверок | match jump table | ⚡ +20% |
| **validate_vm_name()** | несколько проходов | один проход | ⚡ +15% |

### 📊 Метрики рефакторинга

| Метрика | До | После | Улучшение |
|---------|-----|-------|-----------|
| Размер models.rs | ~900 строк | ~700 строк | 📉 -22% |
| Точки валидации | 3 места | 1 место | 📉 -67% |
| Тестовое покрытие | ~60% | ~75% | 📈 +25% |
| Производительность | baseline | оптимизировано | 📈 +20-30% |

### ✅ Обратная совместимость

Все изменения **обратно совместимы**:
- Ре-экспорт старых функций ✅
- Сохранены все публичные API ✅
- Нет breaking changes ✅

---

## 🚀 Roadmap улучшений

### Краткосрочно (v1.1)
- 🔥 Кэширование Azure API calls
- 🔥 Параллельная валидация VM
- 🔥 Улучшенная обработка ошибок

### Среднесрочно (v2.0)
- 🎯 Connection pooling для SSH
- 🎯 Streaming операции
- 🎯 Plugin system

### Долгосрочно (v3.0)
- 🌟 AI integration (NLP queries)
- 🌟 Multi-cloud support (AWS, GCP)
- 🌟 Distributed mode (1000+ VM)

**[Полный Roadmap](rust/IMPROVEMENTS_ROADMAP.md)**

---

## 📚 Документация

### Основы
- **[Быстрый старт](docs/getting-started/quickstart.md)**
- **[Установка](docs/installation.md)**
- **[Аутентификация](docs/AUTH_IMPLEMENTATION_GUIDE.md)**
- **[Конфигурация](docs/reference/config-default-behaviors.md)**

### Фичи
- **[Auto-Sync SSH Keys](docs/features/auto-sync-keys.md)**
- **[Auto-Detect Resource Group](docs/features/auto-detect-rg.md)**
- **[Azure Bastion](docs/features/bastion.md)**
- **[Credential Forwarding](docs/features/credential-forwarding.md)**
- **[NFS Storage](docs/features/nfs-storage.md)**
- **[VM Templates](docs/features/templates.md)**
- **[Snapshots](docs/features/snapshots.md)**

### How-To
- **[Separate Home Disk](docs/how-to/separate-home-disk.md)**
- **[Troubleshoot Connections](docs/how-to/troubleshoot-connection-issues.md)**
- **[Cost Optimization](docs/how-to/cost-optimization.md)**

### Reference
- **[Command Reference](#-основные-команды)**
- **[Config Options](docs/reference/config.md)**
- **[Environment Variables](docs/reference/env-vars.md)**

---

## 🔧 Требования

### Клиентские утилиты
- `az` (Azure CLI)
- `gh` (GitHub CLI)
- `git`
- `ssh`
- `tmux`
- `uv`
- `python`

### Установка зависимостей

**macOS:**
```bash
brew install azure-cli gh git tmux uv python
```

**Linux (Ubuntu/Debian):**
```bash
curl -sL https://aka.ms/InstallAzureCLIDeb | sudo bash
sudo apt install gh git tmux python3
```

**Linux (Fedora/RHEL):**
```bash
sudo dnf install azure-cli gh git tmux python3
```

---

## 💡 Советы и лучшие практики

### Экономия затрат

```bash
# Останавливать VM когда не используются
azlin stop my-vm  # Deallocate, billing stops

# Использовать spot VMs для не критичных workload
azlin new --name spot-worker --vm-size Standard_D2s_v3

# Автоматическая очистка старых VM
azlin prune --age-days 7 --idle-days 3
```

### Производительность

```bash
# Параллельное создание VM
azlin new --pool 5

# Кэширование resource group
# (включено по умолчанию, TTL 15 минут)

# Быстрое подключение через сессии
azlin session my-vm myproject
azlin connect myproject
```

### Безопасность

```bash
# Использовать Bastion для продакшена
azlin new --name prod-vm --no-bastion  # false by default

# Ротация SSH ключей
azlin keys rotate my-vm

# Ограничить доступ через tags
azlin new --name dev-vm --tag env=dev --tag team=backend
```

---

## 🤝 Вклад в проект

### Разработка

```bash
git clone https://github.com/rysweet/azlin
cd azlin/rust
cargo build
cargo test
```

### Структура проекта

```
azlin/
├── rust/                    # Rust код
│   ├── crates/
│   │   ├── azlin/          # Main binary
│   │   ├── azlin-core/     # Core types & config
│   │   ├── azlin-azure/    # Azure SDK wrappers
│   │   ├── azlin-cli/      # CLI commands
│   │   ├── azlin-ssh/      # SSH management
│   │   └── azlin-ai/       # AI/NLP features
│   └── tests/
├── src/                     # Python bridge (legacy)
├── docs/                    # Документация
├── examples/                # Примеры использования
└── scripts/                 # Утилиты
```

### Тестирование

```bash
# Запустить все тесты
cargo test

# Тесты конкретного crate
cargo test -p azlin-core

# Бенчмарки
cargo bench
```

---

## 📄 Лицензия

MIT License — см. [LICENSE](LICENSE) файл.

---

## 🙏 Благодарности

- **Azure SDK for Rust** — Azure integration

---

## 📞 Контакты

- **GitHub**: [rysweet/azlin](https://github.com/rysweet/azlin)
- **GitHub**: [TBoSy-bt/azlin-ref](https://github.com/TBoSy-bt/azlin-ref)

---

<div align="center">

**Сделано с ❤️ на Rust**

[![Rust](https://img.shields.io/badge/rust-1.85+-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

⭐ **Звезда на GitHub помогает проекту расти!** ⭐

</div>
