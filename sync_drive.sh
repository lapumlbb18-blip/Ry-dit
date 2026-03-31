#!/data/data/com.termux/files/usr/bin/bash
# Sincronización RyDit → Google Drive (alucard18)
# Uso: ./sync_drive.sh

set -e

echo "╔════════════════════════════════════════════════════════╗"
echo "║  🛡️  RyDit - Sincronización Google Drive              ║"
echo "╚════════════════════════════════════════════════════════╝"
echo ""

REMOTE="${1:-alucard18}"
DEST="$REMOTE:shield-project"

echo "📁 Origen: $(pwd)"
echo "☁️  Destino: $DEST"
echo ""

# Excluir directorios grandes
EXCLUDES=(
    "--exclude" "target/**"
    "--exclude" ".git/**"
    "--exclude" "*.log"
    "--exclude" "*.log.binaries"
    "--exclude" ".sync_drive.*"
    "--exclude" "__pycache__/**"
)

echo "🔄 Iniciando sincronización..."
echo ""

rclone sync . "$DEST" "${EXCLUDES[@]}" \
    --progress \
    --stats=10s \
    --transfers=4 \
    --checkers=8 \
    2>&1 | tee rydit_sync.log

echo ""
echo "╔════════════════════════════════════════════════════════╗"
echo "║  ✅ Sincronización completada                          ║"
echo "╚════════════════════════════════════════════════════════╝"
