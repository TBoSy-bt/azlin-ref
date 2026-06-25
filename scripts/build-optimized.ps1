#!/usr/bin/env pwsh
# 🔥 Скрипт полной сборки и тестирования azlin v3.0
# 
# Использование: .\scripts\build-optimized.ps1

param(
    [ValidateSet("dev", "dev-fast", "release", "release-small")]
    [string]$Profile = "dev-fast",
    
    [switch]$RunTests,
    [switch]$RunBenchmarks,
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
Write-Host "🔥 AZLIN v3.0 - Оптимизированная Сборка" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# Переход в директорию rust
Push-Location rust

try {
    # Проверка версии Rust
    $rustVersion = rustc --version
    Write-Host "✓ Rust: $rustVersion" -ForegroundColor Green
    
    # Сборка
    if (-not $SkipBuild) {
        Write-Host "`n📦 Сборка профиля: $Profile" -ForegroundColor Yellow
        
        if ($Profile -eq "dev-fast") {
            cargo build --profile dev-fast
        } elseif ($Profile -eq "release-small") {
            cargo build --profile release-small
        } else {
            cargo build --profile $Profile
        }
        
        Write-Host "✓ Сборка завершена" -ForegroundColor Green
    }
    
    # Тесты
    if ($RunTests) {
        Write-Host "`n🧪 Запуск тестов..." -ForegroundColor Yellow
        cargo test --lib
        Write-Host "✓ Тесты пройдены" -ForegroundColor Green
    }
    
    # Бенчмарки
    if ($RunBenchmarks) {
        Write-Host "`n⚡ Запуск бенчмарков..." -ForegroundColor Yellow
        cargo bench
        Write-Host "✓ Бенчмарки завершены" -ForegroundColor Green
    }
    
    # Проверка размера бинарника
    if (-not $SkipBuild) {
        Write-Host "`n📊 Размер бинарника:" -ForegroundColor Yellow
        $binaryPath = "target/$Profile/azlin"
        if (Test-Path $binaryPath) {
            $size = (Get-Item $binaryPath).Length / 1MB
            Write-Host "  azlin: {0:N2} MB" -f $size -ForegroundColor Green
        }
    }
    
    Write-Host "`n✅ ВСЕ ОПЕРАЦИИ ЗАВЕРШЕНЫ!" -ForegroundColor Green
    
} finally {
    Pop-Location
}
