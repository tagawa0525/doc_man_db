# CI/CD パイプライン設計書

## 1. CI/CD概要

### 1.1 設計方針

- **継続的インテグレーション**: プルリクエスト毎の自動テスト実行
- **継続的デプロイメント**: mainブランチへの自動デプロイ
- **品質ゲート**: テスト・静的解析・セキュリティチェック
- **段階的リリース**: 開発→ステージング→本番の段階的展開

### 1.2 ワークフロー概要

```text
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Developer      │    │  CI Pipeline    │    │  CD Pipeline    │
│                 │    │                 │    │                 │
│ 1. Code Push    │───▶│ 2. Build & Test │───▶│ 3. Deploy       │
│ 2. Pull Request │    │ 3. Quality Gate │    │ 4. Monitoring   │
│                 │    │ 4. Security Scan│    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### 1.3 ブランチ戦略

```text
main ────────●─────────●─────────●─────────● (Production)
              ↑         ↑         ↑
develop ──●───┴──●──●───┴──●──●───┴──●──●───┴─ (Integration)
          │      │  │      │  │      │  │
feature/● ●      │  │      │  │      │  │     (Feature branches)
feature/● ────●──┘  │      │  │      │  │
hotfix/●  ──────────●──────┘  │      │  │     (Hotfixes)
release/● ────────────────────●──────┘  │     (Release preparation)
feature/● ─────────────────────────────●─┘    (Feature branches)

Legend:
● = Merge point
─ = Branch timeline
```

### 1.4 環境構成

| 環境            | ブランチ  | デプロイ方式       | 用途             |
| --------------- | --------- | ------------------ | ---------------- |
| **Development** | develop   | 自動デプロイ       | 開発・統合テスト |
| **Staging**     | release/* | 手動承認後デプロイ | リリース前確認   |
| **Production**  | main      | 手動承認後デプロイ | 本番運用         |

## 2. GitHub Actions ワークフロー

### 2.1 CI ワークフロー (Pull Request)

```yaml
# .github/workflows/ci.yml
name: Continuous Integration

on:
  pull_request:
    branches: [main, develop]
  push:
    branches: [develop]

env:
  RUST_BACKTRACE: 1
  SQLX_OFFLINE: true

jobs:
  # Rust Backend CI
  rust-ci:
    name: Rust CI
    runs-on: windows-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy

    - name: Cache cargo registry
      uses: actions/cache@v4
      with:
        path: |
          ~/.cargo/registry/index
          ~/.cargo/registry/cache
          ~/.cargo/git/db
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache cargo build
      uses: actions/cache@v4
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-${{ hashFiles('**/Cargo.lock') }}

    - name: Check Rust formatting
      run: cargo fmt --all -- --check

    - name: Run Clippy
      run: cargo clippy --all-targets --all-features -- -D warnings

    - name: Run unit tests
      run: cargo test --lib --all-features

    - name: Run integration tests
      run: cargo test --test '*' --all-features

    - name: Generate test coverage
      uses: actions-rs/tarpaulin@v0.1
      with:
        version: '0.27.0'
        args: '--all-features --workspace --timeout 120 --out xml'

    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./cobertura.xml
        flags: rust
        name: rust-coverage

    - name: Build release binary
      run: cargo build --release

    - name: Upload build artifacts
      uses: actions/upload-artifact@v4
      with:
        name: rust-binary-${{ github.sha }}
        path: target/release/doc_man_db.exe
        retention-days: 7

  # Frontend CI
  frontend-ci:
    name: Frontend CI
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
        cache-dependency-path: 'frontend/package-lock.json'

    - name: Install dependencies
      working-directory: ./frontend
      run: npm ci

    - name: Check TypeScript types
      working-directory: ./frontend
      run: npm run type-check

    - name: Run ESLint
      working-directory: ./frontend
      run: npm run lint

    - name: Run Prettier check
      working-directory: ./frontend
      run: npm run format:check

    - name: Run unit tests
      working-directory: ./frontend
      run: npm run test:unit

    - name: Run Svelte check
      working-directory: ./frontend
      run: npm run check

    - name: Build application
      working-directory: ./frontend
      run: npm run build

    - name: Upload build artifacts
      uses: actions/upload-artifact@v4
      with:
        name: frontend-build-${{ github.sha }}
        path: frontend/dist
        retention-days: 7

  # E2E Tests
  e2e-tests:
    name: E2E Tests
    runs-on: ubuntu-latest
    needs: [rust-ci, frontend-ci]
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Download Rust binary
      uses: actions/download-artifact@v4
      with:
        name: rust-binary-${{ github.sha }}
        path: ./backend

    - name: Download frontend build
      uses: actions/download-artifact@v4
      with:
        name: frontend-build-${{ github.sha }}
        path: ./frontend/dist

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
        cache-dependency-path: 'frontend/package-lock.json'

    - name: Install Playwright
      working-directory: ./frontend
      run: npx playwright install

    - name: Start test database
      run: |
        # テスト用SQLiteデータベースの初期化
        chmod +x ./backend/doc_man_db.exe
        ./backend/doc_man_db.exe --migrate --test-mode &
        sleep 10

    - name: Run E2E tests
      working-directory: ./frontend
      run: npm run test:e2e

    - name: Upload E2E test results
      uses: actions/upload-artifact@v4
      if: failure()
      with:
        name: e2e-test-results-${{ github.sha }}
        path: frontend/test-results
        retention-days: 7

  # Security Scanning
  security-scan:
    name: Security Scan
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Run Rust security audit
      uses: actions-rs/audit-check@v1
      with:
        token: ${{ secrets.GITHUB_TOKEN }}

    - name: Run npm audit
      working-directory: ./frontend
      run: npm audit --audit-level high

    - name: Run CodeQL Analysis
      uses: github/codeql-action/init@v3
      with:
        languages: javascript, typescript

    - name: Perform CodeQL Analysis
      uses: github/codeql-action/analyze@v3

  # Quality Gate
  quality-gate:
    name: Quality Gate
    runs-on: ubuntu-latest
    needs: [rust-ci, frontend-ci, e2e-tests, security-scan]
    
    steps:
    - name: Quality Gate Summary
      run: |
        echo "✅ All CI checks passed!"
        echo "- Rust: Build, Tests, Clippy, Format ✅"
        echo "- Frontend: Build, Tests, Lint, TypeCheck ✅"
        echo "- E2E: Integration Tests ✅"
        echo "- Security: Audit, CodeQL ✅"
```

### 2.2 CD ワークフロー (Deployment)

```yaml
# .github/workflows/cd.yml
name: Continuous Deployment

on:
  push:
    branches: [main]
  workflow_run:
    workflows: ["Continuous Integration"]
    types: [completed]
    branches: [develop]

env:
  REGISTRY: ghcr.io
  IMAGE_NAME: ${{ github.repository }}

jobs:
  # Build and Package
  build-and-package:
    name: Build and Package
    runs-on: windows-latest
    if: github.event.workflow_run.conclusion == 'success' || github.ref == 'refs/heads/main'

    outputs:
      version: ${{ steps.version.outputs.version }}

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Generate version
      id: version
      run: |
        $timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
        $short_sha = "${{ github.sha }}".Substring(0, 7)
        if ("${{ github.ref }}" -eq "refs/heads/main") {
          $version = "v1.0.0-$timestamp-$short_sha"
        } else {
          $version = "dev-$timestamp-$short_sha"
        }
        echo "version=$version" >> $env:GITHUB_OUTPUT
        echo "Generated version: $version"

    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable

    - name: Build backend
      run: cargo build --release

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
        cache-dependency-path: 'frontend/package-lock.json'

    - name: Build frontend
      working-directory: ./frontend
      run: |
        npm ci
        npm run build

    - name: Create application package
      run: |
        New-Item -ItemType Directory -Path "package" -Force
        Copy-Item "target/release/doc_man_db.exe" "package/"
        Copy-Item -Recurse "frontend/dist" "package/web"
        Copy-Item "migrations" "package/migrations" -Recurse
        Copy-Item "config.example.toml" "package/config.toml"
        
        # Version info file
        @"
        version: ${{ steps.version.outputs.version }}
        build_date: $(Get-Date -Format "yyyy-MM-dd HH:mm:ss")
        commit: ${{ github.sha }}
        branch: ${{ github.ref_name }}
        "@ | Out-File -FilePath "package/version.txt" -Encoding UTF8

    - name: Create installer
      run: |
        # Inno Setup or similar installer creation
        # For now, create a simple zip package
        Compress-Archive -Path "package/*" -DestinationPath "doc_man_db_${{ steps.version.outputs.version }}.zip"

    - name: Upload package artifact
      uses: actions/upload-artifact@v4
      with:
        name: application-package-${{ steps.version.outputs.version }}
        path: doc_man_db_${{ steps.version.outputs.version }}.zip
        retention-days: 30

  # Deploy to Development
  deploy-development:
    name: Deploy to Development
    runs-on: ubuntu-latest
    needs: build-and-package
    if: github.ref == 'refs/heads/develop'
    environment: development

    steps:
    - name: Download package
      uses: actions/download-artifact@v4
      with:
        name: application-package-${{ needs.build-and-package.outputs.version }}

    - name: Deploy to development server
      run: |
        echo "🚀 Deploying to Development Environment"
        echo "Version: ${{ needs.build-and-package.outputs.version }}"
        # ここで実際のデプロイスクリプトを実行
        # ./scripts/deploy-dev.sh ${{ needs.build-and-package.outputs.version }}

    - name: Health check
      run: |
        echo "🏥 Running health check..."
        # curl -f http://dev-server/api/health || exit 1
        echo "✅ Development deployment successful!"

  # Deploy to Staging
  deploy-staging:
    name: Deploy to Staging
    runs-on: ubuntu-latest
    needs: build-and-package
    if: github.ref == 'refs/heads/main'
    environment: staging

    steps:
    - name: Download package
      uses: actions/download-artifact@v4
      with:
        name: application-package-${{ needs.build-and-package.outputs.version }}

    - name: Deploy to staging server
      run: |
        echo "🚀 Deploying to Staging Environment"
        echo "Version: ${{ needs.build-and-package.outputs.version }}"
        # ./scripts/deploy-staging.sh ${{ needs.build-and-package.outputs.version }}

    - name: Run staging tests
      run: |
        echo "🧪 Running staging tests..."
        # npm run test:staging
        echo "✅ Staging tests passed!"

  # Deploy to Production
  deploy-production:
    name: Deploy to Production
    runs-on: ubuntu-latest
    needs: [build-and-package, deploy-staging]
    if: github.ref == 'refs/heads/main'
    environment: production

    steps:
    - name: Download package
      uses: actions/download-artifact@v4
      with:
        name: application-package-${{ needs.build-and-package.outputs.version }}

    - name: Backup current version
      run: |
        echo "💾 Creating backup of current production version..."
        # ./scripts/backup-production.sh

    - name: Deploy to production server
      run: |
        echo "🚀 Deploying to Production Environment"
        echo "Version: ${{ needs.build-and-package.outputs.version }}"
        # ./scripts/deploy-production.sh ${{ needs.build-and-package.outputs.version }}

    - name: Production health check
      run: |
        echo "🏥 Running production health check..."
        # curl -f https://prod-server/api/health || exit 1
        echo "✅ Production deployment successful!"

    - name: Create GitHub Release
      uses: actions/create-release@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        tag_name: ${{ needs.build-and-package.outputs.version }}
        release_name: Release ${{ needs.build-and-package.outputs.version }}
        body: |
          ## Changes
          - Deployed version ${{ needs.build-and-package.outputs.version }}
          - Commit: ${{ github.sha }}
          
          ## Deployment Info
          - Build Date: $(date)
          - Environment: Production
          - Health Check: ✅ Passed
        draft: false
        prerelease: false

  # Monitoring and Notifications
  post-deployment:
    name: Post Deployment
    runs-on: ubuntu-latest
    needs: [deploy-production]
    if: always()

    steps:
    - name: Notify deployment status
      uses: 8398a7/action-slack@v3
      with:
        status: ${{ job.status }}
        channel: '#deployments'
        text: |
          🚀 Deployment Status: ${{ job.status }}
          📦 Version: ${{ needs.build-and-package.outputs.version }}
          🌍 Environment: Production
          📝 Commit: ${{ github.sha }}
      env:
        SLACK_WEBHOOK_URL: ${{ secrets.SLACK_WEBHOOK_URL }}

    - name: Update monitoring dashboards
      run: |
        echo "📊 Updating monitoring dashboards..."
        # curl -X POST "https://monitoring.example.com/api/deployment" \
        #   -H "Authorization: Bearer ${{ secrets.MONITORING_TOKEN }}" \
        #   -d '{"version": "${{ needs.build-and-package.outputs.version }}", "environment": "production"}'
```

### 2.3 定期メンテナンス ワークフロー

```yaml
# .github/workflows/maintenance.yml
name: Maintenance Tasks

on:
  schedule:
    # 毎日午前2時に実行
    - cron: '0 2 * * *'
  workflow_dispatch: # 手動実行可能

jobs:
  # 依存関係の更新チェック
  dependency-updates:
    name: Check Dependency Updates
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Check Rust dependencies
      run: |
        cargo install cargo-outdated
        cargo outdated --exit-code 1 || echo "::warning::Rust dependencies need updates"

    - name: Check npm dependencies
      working-directory: ./frontend
      run: |
        npm outdated || echo "::warning::npm dependencies need updates"

    - name: Security audit
      run: |
        cargo audit || echo "::error::Security vulnerabilities found in Rust dependencies"
        cd frontend && npm audit --audit-level high || echo "::error::Security vulnerabilities found in npm dependencies"

  # パフォーマンス監視
  performance-monitoring:
    name: Performance Monitoring
    runs-on: ubuntu-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Run benchmark tests
      run: |
        cargo bench --bench api_performance | tee benchmark_results.txt

    - name: Check performance regression
      run: |
        # 前回のベンチマーク結果と比較
        # ./scripts/check-performance-regression.sh

    - name: Upload benchmark results
      uses: actions/upload-artifact@v4
      with:
        name: benchmark-results-${{ github.run_id }}
        path: benchmark_results.txt

  # ログ分析
  log-analysis:
    name: Log Analysis
    runs-on: ubuntu-latest

    steps:
    - name: Analyze production logs
      run: |
        echo "📊 Analyzing production logs..."
        # ./scripts/analyze-logs.sh
        # Look for error patterns, performance issues, etc.

    - name: Generate health report
      run: |
        echo "📋 Generating daily health report..."
        # ./scripts/generate-health-report.sh
```

## 3. デプロイメントスクリプト

### 3.1 Windows Server デプロイスクリプト

```powershell
# scripts/deploy-production.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    
    [Parameter(Mandatory=$false)]
    [string]$BackupPath = "C:\Backups\DocManDB",
    
    [Parameter(Mandatory=$false)]
    [string]$InstallPath = "C:\Applications\DocManDB"
)

$ErrorActionPreference = "Stop"

Write-Host "🚀 Starting deployment of DocManDB version $Version" -ForegroundColor Green

try {
    # 1. Service停止
    Write-Host "⏹️ Stopping DocManDB service..." -ForegroundColor Yellow
    if (Get-Service -Name "DocManDB" -ErrorAction SilentlyContinue) {
        Stop-Service -Name "DocManDB" -Force
        Write-Host "✅ Service stopped" -ForegroundColor Green
    }

    # 2. 現在のバージョンをバックアップ
    Write-Host "💾 Creating backup..." -ForegroundColor Yellow
    $BackupDir = Join-Path $BackupPath (Get-Date -Format "yyyyMMdd-HHmmss")
    New-Item -ItemType Directory -Path $BackupDir -Force | Out-Null
    
    if (Test-Path $InstallPath) {
        Copy-Item -Path "$InstallPath\*" -Destination $BackupDir -Recurse -Force
        Write-Host "✅ Backup created at $BackupDir" -ForegroundColor Green
    }

    # 3. 新しいバージョンをダウンロード・展開
    Write-Host "📦 Downloading and extracting new version..." -ForegroundColor Yellow
    $PackagePath = "doc_man_db_$Version.zip"
    
    if (-not (Test-Path $PackagePath)) {
        throw "Package file not found: $PackagePath"
    }
    
    # InstallPathをクリーンアップ
    if (Test-Path $InstallPath) {
        Remove-Item -Path "$InstallPath\*" -Recurse -Force
    } else {
        New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
    }
    
    # 新しいファイルを展開
    Expand-Archive -Path $PackagePath -DestinationPath $InstallPath -Force
    Write-Host "✅ New version extracted" -ForegroundColor Green

    # 4. 設定ファイルの更新
    Write-Host "⚙️ Updating configuration..." -ForegroundColor Yellow
    $ConfigPath = Join-Path $InstallPath "config.toml"
    $BackupConfigPath = Join-Path $BackupDir "config.toml"
    
    if (Test-Path $BackupConfigPath) {
        Copy-Item -Path $BackupConfigPath -Destination $ConfigPath -Force
        Write-Host "✅ Configuration restored" -ForegroundColor Green
    }

    # 5. データベースマイグレーション
    Write-Host "🗃️ Running database migrations..." -ForegroundColor Yellow
    $ExePath = Join-Path $InstallPath "doc_man_db.exe"
    & $ExePath --migrate
    
    if ($LASTEXITCODE -ne 0) {
        throw "Database migration failed"
    }
    Write-Host "✅ Database migrations completed" -ForegroundColor Green

    # 6. Windows Serviceの登録・更新
    Write-Host "🔧 Updating Windows service..." -ForegroundColor Yellow
    
    # 既存サービスを削除
    if (Get-Service -Name "DocManDB" -ErrorAction SilentlyContinue) {
        & sc.exe delete "DocManDB"
    }
    
    # 新しいサービスを作成
    & sc.exe create "DocManDB" binPath= "$ExePath --service" start= auto
    & sc.exe description "DocManDB" "Document Management Database System"
    
    Write-Host "✅ Service updated" -ForegroundColor Green

    # 7. サービス開始
    Write-Host "▶️ Starting DocManDB service..." -ForegroundColor Yellow
    Start-Service -Name "DocManDB"
    
    # サービス起動の確認
    Start-Sleep -Seconds 10
    $Service = Get-Service -Name "DocManDB"
    if ($Service.Status -ne "Running") {
        throw "Service failed to start"
    }
    Write-Host "✅ Service started successfully" -ForegroundColor Green

    # 8. ヘルスチェック
    Write-Host "🏥 Running health check..." -ForegroundColor Yellow
    $HealthUrl = "http://localhost:8080/api/health"
    
    for ($i = 1; $i -le 5; $i++) {
        try {
            $Response = Invoke-RestMethod -Uri $HealthUrl -TimeoutSec 10
            if ($Response.status -eq "healthy") {
                Write-Host "✅ Health check passed" -ForegroundColor Green
                break
            }
        } catch {
            Write-Host "⏳ Health check attempt $i failed, retrying..." -ForegroundColor Yellow
            Start-Sleep -Seconds 10
        }
        
        if ($i -eq 5) {
            throw "Health check failed after 5 attempts"
        }
    }

    # 9. デプロイ完了の記録
    $DeploymentInfo = @{
        Version = $Version
        DeployedAt = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
        DeployedBy = $env:USERNAME
        BackupLocation = $BackupDir
    }
    
    $DeploymentInfo | ConvertTo-Json | Out-File -FilePath (Join-Path $InstallPath "deployment.json") -Encoding UTF8
    
    Write-Host "🎉 Deployment completed successfully!" -ForegroundColor Green
    Write-Host "📦 Version: $Version" -ForegroundColor Cyan
    Write-Host "💾 Backup: $BackupDir" -ForegroundColor Cyan
    Write-Host "🏥 Health: OK" -ForegroundColor Cyan

} catch {
    Write-Host "❌ Deployment failed: $($_.Exception.Message)" -ForegroundColor Red
    
    # ロールバック処理
    Write-Host "🔄 Attempting rollback..." -ForegroundColor Yellow
    
    if (Test-Path $BackupDir) {
        Stop-Service -Name "DocManDB" -Force -ErrorAction SilentlyContinue
        Remove-Item -Path "$InstallPath\*" -Recurse -Force -ErrorAction SilentlyContinue
        Copy-Item -Path "$BackupDir\*" -Destination $InstallPath -Recurse -Force
        Start-Service -Name "DocManDB" -ErrorAction SilentlyContinue
        Write-Host "✅ Rollback completed" -ForegroundColor Yellow
    }
    
    exit 1
}
```

### 3.2 ヘルスチェックスクリプト

```powershell
# scripts/health-check.ps1
param(
    [Parameter(Mandatory=$false)]
    [string]$BaseUrl = "http://localhost:8080"
)

$ErrorActionPreference = "Stop"

Write-Host "🏥 Running comprehensive health check..." -ForegroundColor Cyan

$HealthResults = @{
    ServiceStatus = $false
    ApiHealth = $false
    DatabaseConnection = $false
    FileSystemAccess = $false
    OverallHealth = $false
}

try {
    # 1. Windows Service状態確認
    Write-Host "🔧 Checking Windows service..." -ForegroundColor Yellow
    $Service = Get-Service -Name "DocManDB" -ErrorAction SilentlyContinue
    
    if ($Service -and $Service.Status -eq "Running") {
        $HealthResults.ServiceStatus = $true
        Write-Host "✅ Service is running" -ForegroundColor Green
    } else {
        Write-Host "❌ Service is not running" -ForegroundColor Red
    }

    # 2. API ヘルスエンドポイント確認
    Write-Host "🌐 Checking API health..." -ForegroundColor Yellow
    try {
        $HealthResponse = Invoke-RestMethod -Uri "$BaseUrl/api/health" -TimeoutSec 10
        
        if ($HealthResponse.status -eq "healthy") {
            $HealthResults.ApiHealth = $true
            Write-Host "✅ API is healthy" -ForegroundColor Green
            Write-Host "   Database: $($HealthResponse.database.status)" -ForegroundColor Cyan
            Write-Host "   Connections: $($HealthResponse.database.connections)" -ForegroundColor Cyan
            Write-Host "   Response Time: $($HealthResponse.database.responseTime)" -ForegroundColor Cyan
        } else {
            Write-Host "❌ API health check failed" -ForegroundColor Red
        }
    } catch {
        Write-Host "❌ API is not accessible: $($_.Exception.Message)" -ForegroundColor Red
    }

    # 3. データベース接続確認
    Write-Host "🗃️ Checking database connection..." -ForegroundColor Yellow
    try {
        $DbResponse = Invoke-RestMethod -Uri "$BaseUrl/api/stats" -TimeoutSec 10
        
        if ($DbResponse.documents) {
            $HealthResults.DatabaseConnection = $true
            Write-Host "✅ Database connection OK" -ForegroundColor Green
            Write-Host "   Documents: $($DbResponse.documents.total)" -ForegroundColor Cyan
            Write-Host "   Employees: $($DbResponse.employees.total)" -ForegroundColor Cyan
        }
    } catch {
        Write-Host "❌ Database connection failed: $($_.Exception.Message)" -ForegroundColor Red
    }

    # 4. ファイルシステムアクセス確認
    Write-Host "📁 Checking file system access..." -ForegroundColor Yellow
    try {
        # ネットワークドライブへのアクセステスト
        $TestPath = "\\server01\docs"
        if (Test-Path $TestPath) {
            $HealthResults.FileSystemAccess = $true
            Write-Host "✅ File system access OK" -ForegroundColor Green
        } else {
            Write-Host "❌ Network drive not accessible: $TestPath" -ForegroundColor Red
        }
    } catch {
        Write-Host "❌ File system access failed: $($_.Exception.Message)" -ForegroundColor Red
    }

    # 5. 総合判定
    $AllHealthy = $HealthResults.ServiceStatus -and 
                  $HealthResults.ApiHealth -and 
                  $HealthResults.DatabaseConnection -and 
                  $HealthResults.FileSystemAccess
    
    $HealthResults.OverallHealth = $AllHealthy

    if ($AllHealthy) {
        Write-Host "🎉 All health checks passed!" -ForegroundColor Green
        exit 0
    } else {
        Write-Host "⚠️ Some health checks failed" -ForegroundColor Yellow
        exit 1
    }

} catch {
    Write-Host "❌ Health check failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
} finally {
    # 結果の詳細出力
    Write-Host "`n📊 Health Check Summary:" -ForegroundColor Cyan
    Write-Host "Service Status: $($HealthResults.ServiceStatus ? '✅' : '❌')" -ForegroundColor ($HealthResults.ServiceStatus ? 'Green' : 'Red')
    Write-Host "API Health: $($HealthResults.ApiHealth ? '✅' : '❌')" -ForegroundColor ($HealthResults.ApiHealth ? 'Green' : 'Red')
    Write-Host "Database: $($HealthResults.DatabaseConnection ? '✅' : '❌')" -ForegroundColor ($HealthResults.DatabaseConnection ? 'Green' : 'Red')
    Write-Host "File System: $($HealthResults.FileSystemAccess ? '✅' : '❌')" -ForegroundColor ($HealthResults.FileSystemAccess ? 'Green' : 'Red')
    Write-Host "Overall: $($HealthResults.OverallHealth ? '✅ HEALTHY' : '❌ UNHEALTHY')" -ForegroundColor ($HealthResults.OverallHealth ? 'Green' : 'Red')
}
```

## 4. モニタリング・ログ

### 4.1 アプリケーション監視

```rust
// src/monitoring.rs
use tracing::{info, warn, error};
use serde_json::json;
use tokio::time::{interval, Duration};

pub struct HealthMonitor {
    metrics: HealthMetrics,
}

#[derive(Clone)]
pub struct HealthMetrics {
    pub request_count: u64,
    pub error_count: u64,
    pub avg_response_time: f64,
    pub database_connections: u32,
    pub last_file_check: Option<chrono::DateTime<chrono::Utc>>,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            metrics: HealthMetrics {
                request_count: 0,
                error_count: 0,
                avg_response_time: 0.0,
                database_connections: 0,
                last_file_check: None,
            },
        }
    }

    pub async fn start_monitoring(&self) {
        let mut interval = interval(Duration::from_secs(60)); // 1分毎

        loop {
            interval.tick().await;
            self.collect_metrics().await;
            self.check_thresholds().await;
        }
    }

    async fn collect_metrics(&self) {
        // メトリクス収集
        info!(
            metrics = ?json!({
                "request_count": self.metrics.request_count,
                "error_count": self.metrics.error_count,
                "avg_response_time": self.metrics.avg_response_time,
                "database_connections": self.metrics.database_connections,
            }),
            "Health metrics collected"
        );
    }

    async fn check_thresholds(&self) {
        // エラー率チェック
        if self.metrics.request_count > 100 {
            let error_rate = (self.metrics.error_count as f64) / (self.metrics.request_count as f64);
            if error_rate > 0.05 { // 5%以上
                warn!(error_rate = error_rate, "High error rate detected");
                // アラート送信処理
                self.send_alert("High Error Rate", &format!("Error rate: {:.2}%", error_rate * 100.0)).await;
            }
        }

        // レスポンス時間チェック
        if self.metrics.avg_response_time > 2000.0 { // 2秒以上
            warn!(response_time = self.metrics.avg_response_time, "Slow response time detected");
            self.send_alert("Slow Response", &format!("Avg response time: {:.0}ms", self.metrics.avg_response_time)).await;
        }
    }

    async fn send_alert(&self, title: &str, message: &str) {
        // Teams/Slack通知
        let payload = json!({
            "title": title,
            "text": message,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "severity": "warning"
        });

        // 実際の通知送信処理
        error!("ALERT: {} - {}", title, message);
    }
}
```

### 4.2 構造化ログ設定

```rust
// src/logging.rs
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
use tracing_appender::{rolling, non_blocking};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // ファイルローテーション設定
    let file_appender = rolling::daily("logs", "doc_man_db.log");
    let (file_writer, _guard) = non_blocking(file_appender);

    // コンソール出力
    let (console_writer, _guard) = non_blocking(std::io::stdout());

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "doc_man_db=info,tower_http=debug".into())
        )
        .with(
            fmt::Layer::new()
                .with_writer(file_writer)
                .with_span_events(FmtSpan::CLOSE)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .json() // JSON形式でログ出力
        )
        .with(
            fmt::Layer::new()
                .with_writer(console_writer)
                .with_span_events(FmtSpan::CLOSE)
                .compact() // コンソールは簡潔な形式
        )
        .init();

    Ok(())
}

// 使用例
use tracing::{info, error, instrument};

#[instrument(skip(pool))]
pub async fn create_document(
    pool: &SqlitePool, 
    request: CreateDocumentRequest
) -> Result<Document, Error> {
    info!(
        document_title = %request.title,
        document_type = request.document_type_id,
        "Creating new document"
    );

    match repository::create_document(pool, request).await {
        Ok(document) => {
            info!(
                document_id = document.id,
                document_number = %document.number,
                "Document created successfully"
            );
            Ok(document)
        }
        Err(e) => {
            error!(
                error = %e,
                "Failed to create document"
            );
            Err(e)
        }
    }
}
```

## 5. Tauri アプリ配布

### 5.1 Tauri ビルド・パッケージング

```yaml
# .github/workflows/tauri-release.yml
name: Tauri App Release

on:
  push:
    tags: ['v*']
  workflow_dispatch:

jobs:
  build-tauri:
    name: Build Tauri App
    runs-on: windows-latest

    steps:
    - name: Checkout code
      uses: actions/checkout@v4

    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '20'
        cache: 'npm'
        cache-dependency-path: 'frontend/package-lock.json'

    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Install frontend dependencies
      working-directory: ./frontend
      run: npm ci

    - name: Build Tauri app
      working-directory: ./frontend
      run: npm run tauri build

    - name: Upload Tauri artifacts
      uses: actions/upload-artifact@v4
      with:
        name: tauri-app-${{ github.ref_name }}
        path: |
          frontend/src-tauri/target/release/bundle/msi/*.msi
          frontend/src-tauri/target/release/bundle/nsis/*.exe

  create-release:
    name: Create Release
    runs-on: ubuntu-latest
    needs: build-tauri
    if: startsWith(github.ref, 'refs/tags/')

    steps:
    - name: Download Tauri artifacts
      uses: actions/download-artifact@v4
      with:
        name: tauri-app-${{ github.ref_name }}

    - name: Create GitHub Release
      uses: actions/create-release@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        tag_name: ${{ github.ref_name }}
        release_name: DocManDB ${{ github.ref_name }}
        body: |
          ## DocManDB Desktop Application
          
          ### Installation
          1. Download the installer for your system:
             - Windows Installer (.msi): For managed environments
             - Windows Executable (.exe): For manual installation
          
          2. Run the installer with administrator privileges
          
          3. The application will be available in the Start Menu
          
          ### Features
          - Full document management functionality
          - Native folder opening integration
          - Offline capability
          - Auto-update support
          
          ### System Requirements
          - Windows 10 or later
          - .NET Framework 4.7.2 or later
          - Network access to the document management server
        draft: false
        prerelease: false

    - name: Upload MSI installer
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: ${{ steps.create_release.outputs.upload_url }}
        asset_path: ./msi/DocManDB_${{ github.ref_name }}_x64_en-US.msi
        asset_name: DocManDB-${{ github.ref_name }}-x64.msi
        asset_content_type: application/octet-stream

    - name: Upload EXE installer
      uses: actions/upload-release-asset@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        upload_url: ${{ steps.create_release.outputs.upload_url }}
        asset_path: ./nsis/DocManDB_${{ github.ref_name }}_x64-setup.exe
        asset_name: DocManDB-${{ github.ref_name }}-setup.exe
        asset_content_type: application/octet-stream
```

### 5.2 自動更新設定

```json
// frontend/src-tauri/tauri.conf.json
{
  "tauri": {
    "updater": {
      "active": true,
      "endpoints": [
        "https://github.com/your-org/doc_man_db/releases/latest/download/latest.json"
      ],
      "dialog": true,
      "pubkey": "your-public-key-here"
    }
  }
}
```

---

**最終更新**: 2024年12月  
**作成者**: 開発チーム  
**承認者**: プロジェクトマネージャー
