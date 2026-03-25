# DEMO PROTOCOLO LAZOS v1.0
# Demostración completa del sistema universal de comunicación

echo "============================================================"
echo "🔗 DEMO PROTOCOLO LAZOS v1.0"
echo "============================================================"
echo ""

# 1. Test básico con echo
echo "1. Test básico: system::ping"
echo '{"jsonrpc":"2.0","method":"system::ping","id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result
echo ""

# 2. Bezier lineal
echo "2. Bezier Lineal: (0,0) → (100,100) en t=0.5"
echo '{"jsonrpc":"2.0","method":"science::bezier::linear","params":[0,0,100,100,0.5],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result
echo ""

# 3. Bezier cúbica
echo "3. Bezier Cúbica: P0=(0,0), P1=(30,100), P2=(70,100), P3=(100,0)"
echo "   Generando 5 puntos:"
for t in 0 0.25 0.5 0.75 1; do
    echo -n "   t=$t: "
    echo "{\"jsonrpc\":\"2.0\",\"method\":\"science::bezier::cubic\",\"params\":[0,0,30,100,70,100,100,0,$t],\"id\":1}" | ./target/release/rydit-rs --lazos 2>/dev/null | grep result | cut -d':' -f2-
done
echo ""

# 4. Física: Proyectil
echo "4. Física: Proyectil (v0=50 m/s, angle=45°)"
echo '{"jsonrpc":"2.0","method":"physics::projectile","params":[0,0,50,45],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result | python3 -c "
import sys, json
r = json.loads(sys.stdin.read())['result']
print(f'   Alcance: {r[4]:.2f} m')
print(f'   Altura máxima: {r[3]:.2f} m')
print(f'   Tiempo de vuelo: {r[2]:.2f} s')
"
echo ""

# 5. Física: Gravedad
echo "5. Física: Gravedad (2 cuerpos)"
echo '{"jsonrpc":"2.0","method":"physics::nbody_2","params":[1000,500,0,0,10,0,6.674e-11],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result | python3 -c "
import sys, json
r = json.loads(sys.stdin.read())['result']
print(f'   Fuerza X: {r[0]:.2e} N')
print(f'   Distancia: {r[4]:.2f} m')
"
echo ""

# 6. Estadísticas
echo "6. Estadísticas: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]"
echo -n "   Media: "
echo '{"jsonrpc":"2.0","method":"stats::mean","params":[[1,2,3,4,5,6,7,8,9,10]],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result
echo -n "   Mediana: "
echo '{"jsonrpc":"2.0","method":"stats::median","params":[[1,2,3,4,5,6,7,8,9,10]],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result
echo -n "   Mínimo: "
echo '{"jsonrpc":"2.0","method":"stats::min","params":[[1,2,3,4,5,6,7,8,9,10]],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result
echo -n "   Máximo: "
echo '{"jsonrpc":"2.0","method":"stats::max","params":[[1,2,3,4,5,6,7,8,9,10]],"id":1}' | ./target/release/rydit-rs --lazos 2>/dev/null | grep result
echo ""

# 7. Python bridge
echo "7. Python Bridge: Ejecutando ry_lazo.py"
python3 ry_lazo.py 2>&1 | grep -E "^(1\.|2\.|3\.|4\.|5\.|6\.|=)" | head -20
echo ""

echo "============================================================"
echo "✅ DEMO COMPLETADA"
echo "============================================================"
