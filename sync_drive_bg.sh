#!/bin/bash
# sync_drive_bg.sh - Sincronización en segundo plano a Google Drive
# Usa rclone con .rcloneignore para excluir archivos
#
# Uso:
#   ./sync_drive_bg.sh          # Una sync
#   ./sync_drive_bg.sh --watch  # Continuo cada 5 minutos
#   ./sync_drive_bg.sh --stop   # Detener sync en segundo plano

REMOTE="alucard18:"  # Configurar con: rclone config
DEST="alucard18:Ry-dit-backup"
PROJECT_DIR="/data/data/com.termux/files/home/shield-project"
PID_FILE="$PROJECT_DIR/.sync_drive.pid"
LOG_FILE="$PROJECT_DIR/.sync_drive.log"

# ============================================================================
# DETENER SYNC
# ============================================================================
if [ "$1" = "--stop" ]; then
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        if kill -0 "$PID" 2>/dev/null; then
            kill "$PID"
            echo "✅ Sync detenido (PID: $PID)"
            rm -f "$PID_FILE"
        else
            echo "⚠️  Proceso ya no existía"
            rm -f "$PID_FILE"
        fi
    else
        echo "ℹ️  No hay sync en ejecución"
    fi
    exit 0
fi

# ============================================================================
# VERIFICAR RCLONE
# ============================================================================
if ! command -v rclone &> /dev/null; then
    echo "❌ rclone no instalado"
    echo "   Instalar: pkg install rclone"
    echo "   Configurar: rclone config"
    exit 1
fi

# Verificar remoto configurado
if ! rclone lsd "$REMOTE" &>/dev/null; then
    echo "❌ Remote '$REMOTE' no configurado"
    echo "   Ejecutar: rclone config"
    echo "   Crear remote tipo 'drive' (Google Drive)"
    exit 1
fi

# ============================================================================
# SYNC UNA VEZ
# ============================================================================
sync_once() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] 🔄 Iniciando sync a Google Drive..." >> "$LOG_FILE"
    
    cd "$PROJECT_DIR"
    
    # Crear destino si no existe
    rclone mkdir "$DEST" 2>/dev/null
    
    # Sync con exclusiones
    rclone sync "$PROJECT_DIR" "$DEST" \
        --exclude-from "$PROJECT_DIR/.rcloneignore" \
        --exclude ".git/**" \
        --exclude "target/**" \
        --exclude ".sync_drive.*" \
        --exclude ".sync_drive.pid" \
        --exclude "sync_drive_bg.sh" \
        --progress \
        --log-level INFO \
        --log-file "$LOG_FILE" \
        --transfers 4 \
        --checkers 8 \
        --fast-list \
        --drive-chunk-size 32M \
        2>&1 | tee -a "$LOG_FILE"
    
    EXIT_CODE=$?
    
    if [ $EXIT_CODE -eq 0 ]; then
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] ✅ Sync completado" >> "$LOG_FILE"
    else
        echo "[$(date '+%Y-%m-%d %H:%M:%S')] ❌ Error en sync (código: $EXIT_CODE)" >> "$LOG_FILE"
    fi
    
    return $EXIT_CODE
}

# ============================================================================
# MODO WATCH (CONTINUO CADA 5 MIN)
# ============================================================================
if [ "$1" = "--watch" ]; then
    echo "🔄 Sync en segundo plano cada 5 minutos..."
    echo "   Detener: ./sync_drive_bg.sh --stop"
    echo "   Log: tail -f $LOG_FILE"
    
    # Guardar PID
    echo $$ > "$PID_FILE"
    
    while true; do
        sync_once
        echo "⏳ Próximo sync en 5 minutos..." >> "$LOG_FILE"
        sleep 300  # 5 minutos
    done &
    
    # Guardar PID del background
    echo $! > "$PID_FILE"
    echo "✅ Proceso iniciado (PID: $(cat $PID_FILE))"
    exit 0
fi

# ============================================================================
# MODO NORMAL (UNA VEZ)
# ============================================================================
echo "🔄 Sincronizando a Google Drive..."
sync_once
