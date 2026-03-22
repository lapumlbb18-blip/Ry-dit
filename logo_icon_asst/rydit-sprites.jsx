import { useState } from "react";

const SPRITES = {
  "platform_16x16": "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAXElEQVR4nO2TwQnAQAgE544rwSIszLIsx4ddJY+QPAKBRL+ZpzLLIjgyc6PBVNWOz3J3zAx3rzUAyvIV0KEfEBFlOSKOBpWQ05n3wRcZYD0t3vIfEYaI9L6xIwPsSxEpsRUuwhkAAAAASUVORK5CYII=",
  "crate_8x8":      "iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAAVElEQVR4nGMUERH5z4AHMDEwMDD0RGkw9ERpwAWR+SwwwRuP3jCk2IjA2RpyEDYjzAqYJAzMOfIGYQVRbkAHMDG4CRpyIgxzjrxhmHMEYT+KG3ABAKkfGLuB8485AAAAAElFTkSuQmCC",
  "cube_8x8":       "iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAATElEQVR4nIWPwQ3AMAjEnCgjMA4jMBgDMQJ7pY+qUpOixj84C3FNRCY/DICIKEMzuwWAzFzCZ+7V8s0iqOpH7GzsVwaAu5dPArRTzQv+xhNT5QYSUQAAAABJRU5ErkJggg==",
  "tank_16x16":     "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAApElEQVR4nK2TsRHDIAxFPz6PQJ2aIag9h8bICBkiBXO4Zghq19RpkyL5d7JRzo7Jq8SHLx1IAJ04Bt775xFDrdXptaM5XMOhiuVWVkkczZfgd81LqU2SQR+YJWOW3BipW0WGRvmRUS+mFAEAj/v6EHUNH31sdj6klAAAImLux/hO+t8raL5VJjnnNgE7sL2zpZttPIPZhS3UOUia/lFmcPYzdfMCuVVB1NtlzBUAAAAASUVORK5CYII=",
  "helicopter_16x16": "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAArUlEQVR4nKWTyw3EIAxEB5QSXARFUAnnFLElbBE5pxIXQRHugRwikPksYZV3AY01gy0ZAwURJSwgIibfy4WIkvd+xQ9mLiFGm1cCmLkKMa35dPcZIhdTq+kQq9NP54FDgEOKaaRpbKf8SRUQIgM7ATuVdkeaZmuFdk4ACKsdtHxcnJWfA77RPQZ0I2RWl8qKiGHmauYZ3SLlwqtV1iGz9vPr+jN1EFEa/cpf+msu4qVgTB4sjvUAAAAASUVORK5CYII=",
};

const SPRITE_INFO = [
  { key: "platform_16x16", label: "Plataforma", size: "16×16", desc: "Suelo / terreno" },
  { key: "crate_8x8",      label: "Caja",       size: "8×8",   desc: "Crate de madera" },
  { key: "cube_8x8",       label: "Cubo",       size: "8×8",   desc: "Bloque de piedra" },
  { key: "tank_16x16",     label: "Tanque",     size: "16×16", desc: "Top-down con cañón" },
  { key: "helicopter_16x16", label: "Helicóptero", size: "16×16", desc: "Top-down con rotores" },
];

const SCALES = [1, 2, 4, 8, 16];
const BACKGROUNDS = [
  { id: "dark",    label: "Dark",    bg: "#0a0a12" },
  { id: "mid",     label: "Gray",    bg: "#333344" },
  { id: "light",   label: "Light",   bg: "#e8e8f0" },
  { id: "checker", label: "Alpha",   bg: null },
];

function CheckerBg({ size }) {
  return (
    <div style={{
      position: "absolute", inset: 0,
      backgroundImage: "linear-gradient(45deg,#aaa 25%,transparent 25%),linear-gradient(-45deg,#aaa 25%,transparent 25%),linear-gradient(45deg,transparent 75%,#aaa 75%),linear-gradient(-45deg,transparent 75%,#aaa 75%)",
      backgroundSize: "8px 8px",
      backgroundPosition: "0 0,0 4px,4px -4px,-4px 0",
      backgroundColor: "#fff",
    }} />
  );
}

export default function App() {
  const [scale, setScale] = useState(8);
  const [bg, setBg]       = useState("dark");
  const [active, setActive] = useState("tank_16x16");

  const currentBg = BACKGROUNDS.find(b => b.id === bg);
  const currentSprite = SPRITE_INFO.find(s => s.key === active);

  return (
    <div style={{
      minHeight: "100vh",
      background: "#06060f",
      color: "#d0d0e8",
      fontFamily: "'Courier New', monospace",
      padding: "28px 20px",
      userSelect: "none",
    }}>
      {/* Header */}
      <div style={{ marginBottom: 28 }}>
        <div style={{ display: "flex", alignItems: "center", gap: 10, marginBottom: 4 }}>
          <span style={{ color: "#ff6a00", fontSize: 18, fontWeight: "bold" }}>▸</span>
          <h1 style={{ margin: 0, fontSize: 15, letterSpacing: 5, color: "#fff", textTransform: "uppercase" }}>
            RyDit Sprites
          </h1>
          <span style={{ fontSize: 10, color: "#333", letterSpacing: 2 }}>— DEFAULT ASSETS v0.1</span>
        </div>
        <div style={{ height: 1, background: "linear-gradient(90deg,#ff6a00,#0072ff,transparent)", marginTop: 8 }} />
      </div>

      <div style={{ display: "flex", gap: 20, flexWrap: "wrap" }}>

        {/* Sidebar — sprite list */}
        <div style={{ display: "flex", flexDirection: "column", gap: 4, minWidth: 180 }}>
          <div style={{ fontSize: 9, color: "#444", letterSpacing: 3, marginBottom: 8 }}>SPRITES</div>
          {SPRITE_INFO.map(s => (
            <button key={s.key} onClick={() => setActive(s.key)} style={{
              display: "flex", alignItems: "center", gap: 10,
              padding: "8px 12px",
              background: active === s.key ? "#111122" : "transparent",
              border: "1px solid " + (active === s.key ? "#ff6a00" : "#1a1a2a"),
              color: active === s.key ? "#ff6a00" : "#666",
              cursor: "pointer", textAlign: "left",
              fontFamily: "inherit", fontSize: 11, letterSpacing: 1,
              transition: "all 0.15s",
            }}>
              <img
                src={`data:image/png;base64,${SPRITES[s.key]}`}
                alt={s.label}
                style={{ width: 20, height: 20, imageRendering: "pixelated" }}
              />
              <div>
                <div style={{ fontWeight: "bold" }}>{s.label}</div>
                <div style={{ fontSize: 9, color: "#444", marginTop: 1 }}>{s.size}</div>
              </div>
            </button>
          ))}
        </div>

        {/* Main preview */}
        <div style={{ flex: 1, minWidth: 260 }}>

          {/* Controls */}
          <div style={{ display: "flex", gap: 20, marginBottom: 20, flexWrap: "wrap" }}>
            <div>
              <div style={{ fontSize: 9, color: "#444", letterSpacing: 3, marginBottom: 6 }}>ZOOM</div>
              <div style={{ display: "flex", gap: 4 }}>
                {SCALES.map(s => (
                  <button key={s} onClick={() => setScale(s)} style={{
                    width: 36, height: 28,
                    background: scale === s ? "#ff6a00" : "#0d0d1a",
                    border: "1px solid " + (scale === s ? "#ff6a00" : "#1a1a2a"),
                    color: scale === s ? "#fff" : "#555",
                    cursor: "pointer", fontSize: 10, fontFamily: "inherit",
                  }}>{s}x</button>
                ))}
              </div>
            </div>
            <div>
              <div style={{ fontSize: 9, color: "#444", letterSpacing: 3, marginBottom: 6 }}>FONDO</div>
              <div style={{ display: "flex", gap: 4 }}>
                {BACKGROUNDS.map(b => (
                  <button key={b.id} onClick={() => setBg(b.id)} style={{
                    padding: "5px 10px",
                    background: bg === b.id ? "#0d0d1a" : "transparent",
                    border: "1px solid " + (bg === b.id ? "#00c6ff" : "#1a1a2a"),
                    color: bg === b.id ? "#00c6ff" : "#555",
                    cursor: "pointer", fontSize: 9, fontFamily: "inherit", letterSpacing: 1,
                  }}>{b.label}</button>
                ))}
              </div>
            </div>
          </div>

          {/* Canvas preview */}
          <div style={{
            background: "#0a0a14",
            border: "1px solid #1a1a2a",
            padding: 24,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            gap: 16,
          }}>
            <div style={{ position: "relative", overflow: "hidden", borderRadius: 4 }}>
              {currentBg.bg === null && <CheckerBg />}
              <div style={{
                position: "relative",
                background: currentBg.bg || "transparent",
                padding: 24,
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
              }}>
                <img
                  src={`data:image/png;base64,${SPRITES[active]}`}
                  alt={currentSprite?.label}
                  style={{
                    imageRendering: "pixelated",
                    width:  (parseInt(currentSprite?.size) || 16) * scale,
                    height: (parseInt(currentSprite?.size?.split("×")[1]) || 16) * scale,
                    display: "block",
                  }}
                />
              </div>
            </div>

            <div style={{ textAlign: "center" }}>
              <div style={{ fontSize: 14, color: "#fff", letterSpacing: 2, fontWeight: "bold" }}>
                {currentSprite?.label}
              </div>
              <div style={{ fontSize: 10, color: "#555", marginTop: 4, letterSpacing: 1 }}>
                {currentSprite?.size} · {currentSprite?.desc} · zoom {scale}x → {(parseInt(currentSprite?.size)||16)*scale}px
              </div>
            </div>
          </div>

          {/* Tiled preview */}
          <div style={{ marginTop: 16, background: "#0a0a14", border: "1px solid #1a1a2a", padding: 16 }}>
            <div style={{ fontSize: 9, color: "#444", letterSpacing: 3, marginBottom: 12 }}>TILED PREVIEW (4x zoom)</div>
            <div style={{
              display: "grid",
              gridTemplateColumns: `repeat(8, ${(parseInt(currentSprite?.size)||16)*4}px)`,
              gap: 0,
              width: "fit-content",
            }}>
              {Array.from({length: 16}).map((_,i) => (
                <img key={i}
                  src={`data:image/png;base64,${SPRITES[active]}`}
                  alt=""
                  style={{
                    imageRendering: "pixelated",
                    width:  (parseInt(currentSprite?.size)||16)*4,
                    height: (parseInt(currentSprite?.size?.split("×")[1])||16)*4,
                    display: "block",
                  }}
                />
              ))}
            </div>
          </div>

          {/* xxd command */}
          <div style={{ marginTop: 16, background: "#0a0a14", border: "1px solid #1a1a2a", padding: 16 }}>
            <div style={{ fontSize: 9, color: "#444", letterSpacing: 3, marginBottom: 8 }}>EMBED → C HEADER</div>
            <pre style={{ margin: 0, fontSize: 11, color: "#00ff88", lineHeight: 1.7, whiteSpace: "pre-wrap" }}>
{`# Termux — embed PNG como array C
xxd -i ${active}.png > ${active}.h

# Uso en RyDit:
#include "${active}.h"
// ${active}_png[]  ← array
// ${active}_png_len ← tamaño`}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
}
