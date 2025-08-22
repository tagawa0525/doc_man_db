# Doc Man DB Production Deployment Script
# Version: v1.0.0
# Last Updated: 2024-12-21

param(
    [Parameter(Mandatory=$true)]
    [string]$Environment = "production",
    
    [Parameter(Mandatory=$false)]
    [string]$ServiceName = "DocManDB",
    
    [Parameter(Mandatory=$false)]
    [string]$InstallPath = "C:\doc_man_db",
    
    [Parameter(Mandatory=$false)]
    [string]$BackupPath = "\\backup\docmandb",
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipBackup,
    
    [Parameter(Mandatory=$false)]
    [switch]$SkipMigration,
    
    [Parameter(Mandatory=$false)]
    [switch]$DryRun
)

# Script configuration
$ErrorActionPreference = "Stop"
$VerbosePreference = "Continue"

# Deployment configuration
$DeploymentConfig = @{
    ServiceName = $ServiceName
    InstallPath = $InstallPath
    BackupPath = $BackupPath
    ConfigFile = "config\$Environment.toml"
    BinaryName = "doc_man_db.exe"
    LogPath = "$InstallPath\logs"
    DataPath = "$InstallPath\data"
}

# Logging function
function Write-DeployLog {
    param([string]$Message, [string]$Level = "INFO")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $logMessage = "[$timestamp] [$Level] $Message"
    Write-Host $logMessage
    
    # Also log to file if path exists
    $logFile = "$($DeploymentConfig.LogPath)\deploy.log"
    if (Test-Path (Split-Path $logFile -Parent)) {
        Add-Content -Path $logFile -Value $logMessage
    }
}

# Error handling
function Handle-Error {
    param([string]$Message, [System.Management.Automation.ErrorRecord]$Error)
    Write-DeployLog "ERROR: $Message" "ERROR"
    if ($Error) {
        Write-DeployLog "Exception: $($Error.Exception.Message)" "ERROR"
    }
    exit 1
}

# Pre-deployment checks
function Test-Prerequisites {
    Write-DeployLog "Performing pre-deployment checks..."
    
    # Check if running as administrator
    $currentPrincipal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    if (-not $currentPrincipal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        Handle-Error "This script must be run as Administrator"
    }
    
    # Check binary exists
    if (-not (Test-Path ".\$($DeploymentConfig.BinaryName)")) {
        Handle-Error "Binary file $($DeploymentConfig.BinaryName) not found in current directory"
    }
    
    # Check configuration file exists
    if (-not (Test-Path $DeploymentConfig.ConfigFile)) {
        Handle-Error "Configuration file $($DeploymentConfig.ConfigFile) not found"
    }
    
    # Test database connectivity
    Write-DeployLog "Testing database connectivity..."
    try {
        $configContent = Get-Content $DeploymentConfig.ConfigFile -Raw
        if ($configContent -match 'url\s*=\s*"([^"]+)"') {
            $dbUrl = $matches[1]
            Write-DeployLog "Database URL found: $($dbUrl -replace 'password=[^;]+', 'password=***')"
        }
    }
    catch {
        Write-DeployLog "Warning: Could not parse database URL from config" "WARN"
    }
    
    # Check disk space (minimum 1GB)
    $drive = Split-Path $DeploymentConfig.InstallPath -Qualifier
    $freeSpace = (Get-WmiObject -Class Win32_LogicalDisk -Filter "DeviceID='$drive'").FreeSpace / 1GB
    if ($freeSpace -lt 1) {
        Handle-Error "Insufficient disk space. Available: $([math]::Round($freeSpace, 2))GB, Required: 1GB"
    }
    
    Write-DeployLog "Pre-deployment checks passed"
}

# Create backup
function New-Backup {
    if ($SkipBackup) {
        Write-DeployLog "Skipping backup as requested"
        return
    }
    
    Write-DeployLog "Creating backup..."
    
    $backupTimestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $backupFolder = "$($DeploymentConfig.BackupPath)\backup_$backupTimestamp"
    
    try {
        # Create backup directory
        New-Item -ItemType Directory -Path $backupFolder -Force | Out-Null
        
        # Backup current installation if exists
        if (Test-Path $DeploymentConfig.InstallPath) {
            Write-DeployLog "Backing up current installation to $backupFolder"
            Copy-Item -Path "$($DeploymentConfig.InstallPath)\*" -Destination $backupFolder -Recurse -Force
        }
        
        # Backup database if it's SQLite
        $dbFile = "$($DeploymentConfig.DataPath)\production.db"
        if (Test-Path $dbFile) {
            Write-DeployLog "Backing up SQLite database"
            Copy-Item -Path $dbFile -Destination "$backupFolder\production_backup.db" -Force
        }
        
        Write-DeployLog "Backup completed: $backupFolder"
    }
    catch {
        Handle-Error "Backup failed" $_
    }
}

# Stop service
function Stop-DocManService {
    Write-DeployLog "Stopping $($DeploymentConfig.ServiceName) service..."
    
    $service = Get-Service -Name $DeploymentConfig.ServiceName -ErrorAction SilentlyContinue
    if ($service) {
        if ($service.Status -eq "Running") {
            if (-not $DryRun) {
                Stop-Service -Name $DeploymentConfig.ServiceName -Force
                
                # Wait for service to stop
                $timeout = 30
                $counter = 0
                do {
                    Start-Sleep -Seconds 1
                    $service = Get-Service -Name $DeploymentConfig.ServiceName
                    $counter++
                } while ($service.Status -ne "Stopped" -and $counter -lt $timeout)
                
                if ($service.Status -ne "Stopped") {
                    Handle-Error "Service did not stop within $timeout seconds"
                }
            }
            Write-DeployLog "Service stopped successfully"
        }
        else {
            Write-DeployLog "Service is already stopped"
        }
    }
    else {
        Write-DeployLog "Service not found, will create new service"
    }
}

# Deploy files
function Deploy-Files {
    Write-DeployLog "Deploying files to $($DeploymentConfig.InstallPath)..."
    
    try {
        # Create directories
        $directories = @(
            $DeploymentConfig.InstallPath,
            "$($DeploymentConfig.InstallPath)\config",
            "$($DeploymentConfig.InstallPath)\logs", 
            "$($DeploymentConfig.InstallPath)\data",
            "$($DeploymentConfig.InstallPath)\temp",
            "$($DeploymentConfig.InstallPath)\certs"
        )
        
        foreach ($dir in $directories) {
            if (-not (Test-Path $dir)) {
                Write-DeployLog "Creating directory: $dir"
                if (-not $DryRun) {
                    New-Item -ItemType Directory -Path $dir -Force | Out-Null
                }
            }
        }
        
        # Deploy binary
        Write-DeployLog "Deploying binary: $($DeploymentConfig.BinaryName)"
        if (-not $DryRun) {
            Copy-Item -Path ".\$($DeploymentConfig.BinaryName)" -Destination "$($DeploymentConfig.InstallPath)\$($DeploymentConfig.BinaryName)" -Force
        }
        
        # Deploy configuration
        Write-DeployLog "Deploying configuration: $($DeploymentConfig.ConfigFile)"
        if (-not $DryRun) {
            Copy-Item -Path $DeploymentConfig.ConfigFile -Destination "$($DeploymentConfig.InstallPath)\config\$Environment.toml" -Force
        }
        
        # Deploy additional files
        $additionalFiles = @("README.md", "RELEASE.md")
        foreach ($file in $additionalFiles) {
            if (Test-Path $file) {
                Write-DeployLog "Deploying: $file"
                if (-not $DryRun) {
                    Copy-Item -Path $file -Destination "$($DeploymentConfig.InstallPath)\$file" -Force
                }
            }
        }
        
        # Set permissions
        Write-DeployLog "Setting file permissions..."
        if (-not $DryRun) {
            icacls "$($DeploymentConfig.InstallPath)" /grant "NT SERVICE\$($DeploymentConfig.ServiceName):(OI)(CI)F" /T | Out-Null
        }
        
        Write-DeployLog "File deployment completed"
    }
    catch {
        Handle-Error "File deployment failed" $_
    }
}

# Run database migration
function Invoke-Migration {
    if ($SkipMigration) {
        Write-DeployLog "Skipping database migration as requested"
        return
    }
    
    Write-DeployLog "Running database migration..."
    
    try {
        $migrationCmd = "$($DeploymentConfig.InstallPath)\$($DeploymentConfig.BinaryName) --config config\$Environment.toml --migrate"
        
        if ($DryRun) {
            Write-DeployLog "DRY RUN: Would execute: $migrationCmd"
        }
        else {
            $result = & cmd /c "cd /d `"$($DeploymentConfig.InstallPath)`" && $migrationCmd" 2>&1
            
            if ($LASTEXITCODE -eq 0) {
                Write-DeployLog "Database migration completed successfully"
            }
            else {
                Write-DeployLog "Migration output: $result" "WARN"
                Handle-Error "Database migration failed with exit code $LASTEXITCODE"
            }
        }
    }
    catch {
        Handle-Error "Database migration failed" $_
    }
}

# Install/update Windows service
function Install-WindowsService {
    Write-DeployLog "Installing Windows service..."
    
    $servicePath = "$($DeploymentConfig.InstallPath)\$($DeploymentConfig.BinaryName)"
    $serviceCommand = "`"$servicePath`" --service --config config\$Environment.toml"
    
    try {
        # Check if service exists
        $service = Get-Service -Name $DeploymentConfig.ServiceName -ErrorAction SilentlyContinue
        
        if ($service) {
            Write-DeployLog "Updating existing service configuration"
            if (-not $DryRun) {
                sc.exe config $DeploymentConfig.ServiceName binPath= $serviceCommand | Out-Null
                if ($LASTEXITCODE -ne 0) {
                    Handle-Error "Failed to update service configuration"
                }
            }
        }
        else {
            Write-DeployLog "Creating new Windows service"
            if (-not $DryRun) {
                sc.exe create $DeploymentConfig.ServiceName binPath= $serviceCommand start= auto displayname= "Document Management Database Service" | Out-Null
                if ($LASTEXITCODE -ne 0) {
                    Handle-Error "Failed to create Windows service"
                }
                
                # Set service description
                sc.exe description $DeploymentConfig.ServiceName "Document Management System with circulation workflow support" | Out-Null
            }
        }
        
        # Configure service recovery
        if (-not $DryRun) {
            sc.exe failure $DeploymentConfig.ServiceName reset= 86400 actions= restart/30000/restart/60000/restart/120000 | Out-Null
        }
        
        Write-DeployLog "Service installation completed"
    }
    catch {
        Handle-Error "Service installation failed" $_
    }
}

# Start service
function Start-DocManService {
    Write-DeployLog "Starting $($DeploymentConfig.ServiceName) service..."
    
    if ($DryRun) {
        Write-DeployLog "DRY RUN: Would start the service"
        return
    }
    
    try {
        Start-Service -Name $DeploymentConfig.ServiceName
        
        # Wait for service to start
        $timeout = 60
        $counter = 0
        do {
            Start-Sleep -Seconds 1
            $service = Get-Service -Name $DeploymentConfig.ServiceName
            $counter++
        } while ($service.Status -ne "Running" -and $counter -lt $timeout)
        
        if ($service.Status -eq "Running") {
            Write-DeployLog "Service started successfully"
        }
        else {
            Handle-Error "Service did not start within $timeout seconds"
        }
    }
    catch {
        Handle-Error "Failed to start service" $_
    }
}

# Verify deployment
function Test-Deployment {
    Write-DeployLog "Verifying deployment..."
    
    # Check service status
    $service = Get-Service -Name $DeploymentConfig.ServiceName -ErrorAction SilentlyContinue
    if ($service -and $service.Status -eq "Running") {
        Write-DeployLog "✓ Service is running"
    }
    else {
        Write-DeployLog "✗ Service is not running" "ERROR"
        return $false
    }
    
    # Check HTTP endpoint (basic connectivity)
    try {
        Start-Sleep -Seconds 5  # Give service time to fully initialize
        $response = Invoke-WebRequest -Uri "http://localhost:8080/api/v1/system/health" -TimeoutSec 10 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            Write-DeployLog "✓ HTTP endpoint responding"
        }
        else {
            Write-DeployLog "✗ HTTP endpoint returned status $($response.StatusCode)" "WARN"
        }
    }
    catch {
        Write-DeployLog "✗ HTTP endpoint not responding: $($_.Exception.Message)" "WARN"
    }
    
    # Check log files
    $logFile = "$($DeploymentConfig.LogPath)\app.log"
    if (Test-Path $logFile) {
        Write-DeployLog "✓ Log file created"
        
        # Check for errors in recent logs
        $recentLogs = Get-Content $logFile -Tail 50 | Where-Object { $_ -match "ERROR" }
        if ($recentLogs) {
            Write-DeployLog "⚠ Found recent errors in logs:" "WARN"
            $recentLogs | ForEach-Object { Write-DeployLog "  $_" "WARN" }
        }
    }
    else {
        Write-DeployLog "⚠ Log file not found" "WARN"
    }
    
    return $true
}

# Main deployment process
function Start-Deployment {
    Write-DeployLog "Starting deployment of Doc Man DB v1.0.0 to $Environment environment"
    Write-DeployLog "Target: $($DeploymentConfig.InstallPath)"
    
    if ($DryRun) {
        Write-DeployLog "DRY RUN MODE - No changes will be made" "WARN"
    }
    
    try {
        # Pre-deployment checks
        Test-Prerequisites
        
        # Create backup
        New-Backup
        
        # Stop existing service
        Stop-DocManService
        
        # Deploy files
        Deploy-Files
        
        # Run database migration
        Invoke-Migration
        
        # Install/update service
        Install-WindowsService
        
        # Start service
        Start-DocManService
        
        # Verify deployment
        $verificationPassed = Test-Deployment
        
        if ($verificationPassed) {
            Write-DeployLog "🎉 Deployment completed successfully!" "SUCCESS"
            Write-DeployLog "Service is running at: http://localhost:8080"
            Write-DeployLog "Health check: http://localhost:8080/api/v1/system/health"
        }
        else {
            Write-DeployLog "⚠ Deployment completed with warnings" "WARN"
        }
        
        # Display post-deployment information
        Write-DeployLog ""
        Write-DeployLog "=== POST-DEPLOYMENT INFORMATION ==="
        Write-DeployLog "Service Name: $($DeploymentConfig.ServiceName)"
        Write-DeployLog "Install Path: $($DeploymentConfig.InstallPath)"
        Write-DeployLog "Config File: $($DeploymentConfig.InstallPath)\config\$Environment.toml"
        Write-DeployLog "Log Files: $($DeploymentConfig.LogPath)"
        Write-DeployLog ""
        Write-DeployLog "Next Steps:"
        Write-DeployLog "1. Verify the application is accessible"
        Write-DeployLog "2. Test authentication with your credentials"
        Write-DeployLog "3. Configure SSL certificates if needed"
        Write-DeployLog "4. Set up monitoring and alerting"
        Write-DeployLog "5. Schedule regular backups"
        Write-DeployLog ""
        Write-DeployLog "For support, see: docs/troubleshooting.md"
    }
    catch {
        Handle-Error "Deployment failed" $_
    }
}

# Script entry point
Write-Host "Doc Man DB Production Deployment Script v1.0.0" -ForegroundColor Green
Write-Host "=============================================" -ForegroundColor Green
Write-Host ""

# Start deployment
Start-Deployment