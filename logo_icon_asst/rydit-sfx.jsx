import { useState, useRef, useCallback, useEffect } from "react";

// ─── SPRITES BASE64 (thumbnails) ───────────────────────────────────────────
const SPRITES = {
  platform:    "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAXElEQVR4nO2TwQnAQAgE544rwSIszLIsx4ddJY+QPAKBRL+ZpzLLIjgyc6PBVNWOz3J3zAx3rzUAyvIV0KEfEBFlOSKOBpWQ05n3wRcZYD0t3vIfEYaI9L6xIwPsSxEpsRUuwhkAAAAASUVORK5CYII=",
  crate:       "iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAAVElEQVR4nGMUERH5z4AHMDEwMDD0RGkw9ERpwAWR+SwwwRuP3jCk2IjA2RpyEDYjzAqYJAzMOfIGYQVRbkAHMDG4CRpyIgxzjrxhmHMEYT+KG3ABAKkfGLuB8485AAAAAElFTkSuQmCC",
  cube:        "iVBORw0KGgoAAAANSUhEUgAAAAgAAAAICAYAAADED76LAAAATElEQVR4nIWPwQ3AMAjEnCgjMA4jMBgDMQJ7pY+qUpOixj84C3FNRCY/DICIKEMzuwWAzFzCZ+7V8s0iqOpH7GzsVwaAu5dPArRTzQv+xhNT5QYSUQAAAABJRU5ErkJggg==",
  tank:        "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAApElEQVR4nK2TsRHDIAxFPz6PQJ2aIag9h8bICBkiBXO4Zghq19RpkyL5d7JRzo7Jq8SHLx1IAJ04Bt775xFDrdXptaM5XMOhiuVWVkkczZfgd81LqU2SQR+YJWOW3BipW0WGRvmRUS+mFAEAj/v6EHUNH31sdj6klAAAImLux/hO+t8raL5VJjnnNgE7sL2zpZttPIPZhS3UOUia/lFmcPYzdfMCuVVB1NtlzBUAAAAASUVORK5CYII=",
  helicopter:  "iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAArUlEQVR4nKWTyw3EIAxEB5QSXARFUAnnFLElbBE5pxIXQRHugRwikPksYZV3AY01gy0ZAwURJSwgIibfy4WIkvd+xQ9mLiFGm1cCmLkKMa35dPcZIhdTq+kQq9NP54FDgEOKaaRpbKf8SRUQIgM7ATuVdkeaZmuFdk4ACKsdtHxcnJWfA77RPQZ0I2RWl8qKiGHmauYZ3SLlwqtV1iGz9vPr+jN1EFEa/cpf+msu4qVgTB4sjvUAAAAASUVORK5CYII=",
};

// ─── SOUND PRESETS ─────────────────────────────────────────────────────────
const PRESETS = {
  platform: [
    { id:"land",    label:"Aterrizar",  wave:"noise", freq:80,   attack:0.001, decay:0.08, sustain:0.0, release:0.05, pitch_sweep:-0.3, volume:0.6 },
    { id:"step",    label:"Paso",       wave:"noise", freq:120,  attack:0.001, decay:0.04, sustain:0.0, release:0.02, pitch_sweep:0,    volume:0.3 },
  ],
  crate: [
    { id:"hit",     label:"Golpe",      wave:"noise", freq:200,  attack:0.001, decay:0.12, sustain:0.0, release:0.08, pitch_sweep:-0.5, volume:0.7 },
    { id:"break",   label:"Romper",     wave:"noise", freq:150,  attack:0.001, decay:0.25, sustain:0.0, release:0.15, pitch_sweep:-0.8, volume:0.9 },
  ],
  cube: [
    { id:"stone",   label:"Piedra",     wave:"sine",  freq:180,  attack:0.001, decay:0.15, sustain:0.0, release:0.1,  pitch_sweep:-0.4, volume:0.6 },
    { id:"bounce",  label:"Rebote",     wave:"sine",  freq:320,  attack:0.001, decay:0.1,  sustain:0.0, release:0.06, pitch_sweep:0.2,  volume:0.5 },
  ],
  tank: [
    { id:"engine",  label:"Motor",      wave:"sawtooth", freq:55, attack:0.05, decay:0.1,  sustain:0.7, release:0.2,  pitch_sweep:0,    volume:0.5 },
    { id:"shoot",   label:"Disparo",    wave:"sawtooth", freq:120,attack:0.001,decay:0.08, sustain:0.0, release:0.12, pitch_sweep:-0.9, volume:1.0 },
    { id:"explode", label:"Explosión",  wave:"noise", freq:60,   attack:0.001, decay:0.4,  sustain:0.0, release:0.3,  pitch_sweep:-0.6, volume:1.0 },
  ],
  helicopter: [
    { id:"rotor",   label:"Rotor",      wave:"sawtooth", freq:22, attack:0.1,  decay:0.0,  sustain:0.8, release:0.3,  pitch_sweep:0,    volume:0.4 },
    { id:"shoot2",  label:"Ráfaga",     wave:"square",freq:440,  attack:0.001, decay:0.03, sustain:0.0, release:0.02, pitch_sweep:0.1,  volume:0.7 },
    { id:"explode2",label:"Explosión",  wave:"noise", freq:50,   attack:0.001, decay:0.5,  sustain:0.0, release:0.4,  pitch_sweep:-0.5, volume:1.0 },
  ],
};

const ENTITIES = [
  { key:"platform",   label:"Plataforma", color:"#4a9eff" },
  { key:"crate",      label:"Caja",       color:"#c8843a" },
  { key:"cube",       label:"Cubo",       color:"#aaaaaa" },
  { key:"tank",       label:"Tanque",     color:"#5ab85a" },
  { key:"helicopter", label:"Helicóptero",color:"#50c8cc" },
];

// ─── AUDIO ENGINE ──────────────────────────────────────────────────────────
function playSound(params, audioCtx) {
  const { wave, freq, attack, decay, sustain, release, pitch_sweep, volume } = params;
  const now = audioCtx.currentTime;
  const duration = attack + decay + (sustain > 0 ? 0.15 : 0) + release;

  const gainNode = audioCtx.createGain();
  gainNode.connect(audioCtx.destination);

  // ADSR envelope
  gainNode.gain.setValueAtTime(0, now);
  gainNode.gain.linearRampToValueAtTime(volume, now + attack);
  gainNode.gain.linearRampToValueAtTime(volume * sustain, now + attack + decay);
  gainNode.gain.setValueAtTime(volume * sustain, now + attack + decay + (sustain > 0 ? 0.15 : 0));
  gainNode.gain.linearRampToValueAtTime(0.0001, now + duration);

  if (wave === "noise") {
    // White noise filtered
    const bufSize = audioCtx.sampleRate * duration;
    const buffer = audioCtx.createBuffer(1, bufSize, audioCtx.sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < bufSize; i++) data[i] = Math.random() * 2 - 1;
    const source = audioCtx.createBufferSource();
    source.buffer = buffer;

    const filter = audioCtx.createBiquadFilter();
    filter.type = "bandpass";
    filter.frequency.setValueAtTime(freq, now);
    filter.frequency.linearRampToValueAtTime(freq * (1 + pitch_sweep * 3), now + duration);
    filter.Q.value = 1.5;

    source.connect(filter);
    filter.connect(gainNode);
    source.start(now);
    source.stop(now + duration + 0.1);
  } else {
    const osc = audioCtx.createOscillator();
    osc.type = wave;
    osc.frequency.setValueAtTime(freq, now);
    osc.frequency.linearRampToValueAtTime(freq * Math.pow(2, pitch_sweep * 2), now + duration);
    osc.connect(gainNode);
    osc.start(now);
    osc.stop(now + duration + 0.1);
  }

  return duration;
}

// ─── WAVEFORM VISUALIZER ───────────────────────────────────────────────────
function WaveformViz({ params }) {
  const canvasRef = useRef(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const W = canvas.width, H = canvas.height;
    ctx.clearRect(0, 0, W, H);

    // Draw envelope shape
    const { attack, decay, sustain, release, volume } = params;
    const total = attack + decay + 0.15 + release;
    const toX = t => (t / total) * W;

    ctx.beginPath();
    ctx.moveTo(0, H);
    ctx.lineTo(toX(attack), H * (1 - volume));
    ctx.lineTo(toX(attack + decay), H * (1 - volume * sustain));
    ctx.lineTo(toX(attack + decay + 0.15), H * (1 - volume * sustain));
    ctx.lineTo(W, H);
    ctx.closePath();

    const grad = ctx.createLinearGradient(0, 0, W, 0);
    grad.addColorStop(0, "rgba(255,106,0,0.8)");
    grad.addColorStop(0.5, "rgba(0,198,255,0.6)");
    grad.addColorStop(1, "rgba(0,198,255,0.1)");
    ctx.fillStyle = grad;
    ctx.fill();

    ctx.strokeStyle = "#ff6a00";
    ctx.lineWidth = 1.5;
    ctx.stroke();

    // Grid
    ctx.strokeStyle = "rgba(255,255,255,0.05)";
    ctx.lineWidth = 1;
    for (let i = 1; i < 4; i++) {
      ctx.beginPath(); ctx.moveTo(W*i/4, 0); ctx.lineTo(W*i/4, H); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(0, H*i/4); ctx.lineTo(W, H*i/4); ctx.stroke();
    }

    // Labels
    ctx.fillStyle = "rgba(255,106,0,0.7)";
    ctx.font = "9px monospace";
    ctx.fillText("A", toX(attack/2), H - 4);
    ctx.fillText("D", toX(attack + decay/2), H - 4);
    ctx.fillText("S", toX(attack + decay + 0.075), H - 4);
    ctx.fillText("R", toX(attack + decay + 0.15 + release/2), H - 4);
  }, [params]);

  return <canvas ref={canvasRef} width={260} height={60}
    style={{ display:"block", width:"100%", height:60, borderRadius:2 }} />;
}

// ─── SLIDER ────────────────────────────────────────────────────────────────
function Knob({ label, value, min, max, step, onChange, color="#ff6a00", format }) {
  return (
    <div style={{ display:"flex", flexDirection:"column", gap:3 }}>
      <div style={{ display:"flex", justifyContent:"space-between", alignItems:"center" }}>
        <span style={{ fontSize:9, color:"#555", letterSpacing:2 }}>{label}</span>
        <span style={{ fontSize:10, color, fontWeight:"bold" }}>
          {format ? format(value) : value}
        </span>
      </div>
      <input type="range" min={min} max={max} step={step} value={value}
        onChange={e => onChange(parseFloat(e.target.value))}
        style={{ width:"100%", accentColor: color, height:3, cursor:"pointer" }}
      />
    </div>
  );
}

// ─── MAIN ──────────────────────────────────────────────────────────────────
export default function App() {
  const [entity, setEntity]     = useState("tank");
  const [sfxIdx, setSfxIdx]     = useState(0);
  const [playing, setPlaying]   = useState(false);
  const [showCode, setShowCode] = useState(false);
  const audioCtxRef = useRef(null);

  const presets = PRESETS[entity];
  const basePreset = presets[Math.min(sfxIdx, presets.length-1)];
  const [params, setParams] = useState({ ...basePreset });

  // Sync when entity/sfx changes
  useEffect(() => {
    const p = PRESETS[entity][Math.min(sfxIdx, PRESETS[entity].length-1)];
    setParams({ ...p });
  }, [entity, sfxIdx]);

  const updateParam = (key, val) => setParams(p => ({ ...p, [key]: val }));

  const play = useCallback(() => {
    if (!audioCtxRef.current) audioCtxRef.current = new (window.AudioContext || window.webkitAudioContext)();
    const dur = playSound(params, audioCtxRef.current);
    setPlaying(true);
    setTimeout(() => setPlaying(false), dur * 1000 + 100);
  }, [params]);

  const currentEntity = ENTITIES.find(e => e.key === entity);
  const sfxList = PRESETS[entity];

  const cCode = `// RyDit Engine — SFX params
typedef struct {
    RyditWave wave;    // ${params.wave.toUpperCase()}
    float freq;        // ${params.freq.toFixed(1)} Hz
    float attack;      // ${params.attack.toFixed(3)} s
    float decay;       // ${params.decay.toFixed(3)} s
    float sustain;     // ${params.sustain.toFixed(2)}
    float release;     // ${params.release.toFixed(3)} s
    float pitch_sweep; // ${params.pitch_sweep.toFixed(2)}
    float volume;      // ${params.volume.toFixed(2)}
} RyditSFX;

static const RyditSFX SFX_${entity.toUpperCase()}_${params.id?.toUpperCase()} = {
    .wave        = WAVE_${params.wave.toUpperCase()},
    .freq        = ${params.freq.toFixed(1)}f,
    .attack      = ${params.attack.toFixed(3)}f,
    .decay       = ${params.decay.toFixed(3)}f,
    .sustain     = ${params.sustain.toFixed(2)}f,
    .release     = ${params.release.toFixed(3)}f,
    .pitch_sweep = ${params.pitch_sweep.toFixed(2)}f,
    .volume      = ${params.volume.toFixed(2)}f,
};

// Uso:
// rydit_sfx_play(SFX_${entity.toUpperCase()}_${params.id?.toUpperCase()});`;

  return (
    <div style={{
      minHeight:"100vh", background:"#06060f", color:"#d0d0e8",
      fontFamily:"'Courier New', monospace", padding:"24px 18px",
    }}>
      {/* Header */}
      <div style={{ marginBottom:24 }}>
        <div style={{ display:"flex", alignItems:"center", gap:10, marginBottom:4 }}>
          <span style={{ color:"#ff6a00", fontSize:20 }}>◈</span>
          <h1 style={{ margin:0, fontSize:14, letterSpacing:5, color:"#fff", textTransform:"uppercase" }}>
            RyDit SFX Synth
          </h1>
          <span style={{ fontSize:9, color:"#333", letterSpacing:2 }}>— PROCEDURAL AUDIO v0.1</span>
        </div>
        <div style={{ height:1, background:"linear-gradient(90deg,#ff6a00,#0072ff,transparent)" }} />
      </div>

      <div style={{ display:"flex", gap:16, flexWrap:"wrap" }}>

        {/* LEFT — entity selector */}
        <div style={{ display:"flex", flexDirection:"column", gap:3, minWidth:150 }}>
          <div style={{ fontSize:9, color:"#444", letterSpacing:3, marginBottom:6 }}>ENTIDAD</div>
          {ENTITIES.map(e => (
            <button key={e.key} onClick={() => { setEntity(e.key); setSfxIdx(0); }}
              style={{
                display:"flex", alignItems:"center", gap:8, padding:"7px 10px",
                background: entity===e.key ? "#0d0d1a" : "transparent",
                border:"1px solid "+(entity===e.key ? e.color : "#1a1a2a"),
                color: entity===e.key ? e.color : "#555",
                cursor:"pointer", fontFamily:"inherit", fontSize:11, letterSpacing:1,
                transition:"all 0.12s",
              }}>
              <img src={`data:image/png;base64,${SPRITES[e.key]}`} alt={e.label}
                style={{ width:18, height:18, imageRendering:"pixelated" }} />
              {e.label}
            </button>
          ))}
        </div>

        {/* CENTER — SFX list + controls */}
        <div style={{ flex:1, minWidth:240 }}>

          {/* SFX tabs */}
          <div style={{ fontSize:9, color:"#444", letterSpacing:3, marginBottom:6 }}>SONIDOS</div>
          <div style={{ display:"flex", gap:4, marginBottom:16, flexWrap:"wrap" }}>
            {sfxList.map((s,i) => (
              <button key={s.id} onClick={() => setSfxIdx(i)}
                style={{
                  padding:"6px 12px",
                  background: sfxIdx===i ? currentEntity.color : "#0d0d1a",
                  border:"1px solid "+(sfxIdx===i ? currentEntity.color : "#1a1a2a"),
                  color: sfxIdx===i ? "#000" : "#666",
                  cursor:"pointer", fontFamily:"inherit", fontSize:10, letterSpacing:1,
                  fontWeight: sfxIdx===i ? "bold" : "normal",
                }}>
                {s.label}
              </button>
            ))}
          </div>

          {/* Waveform */}
          <div style={{ background:"#0a0a14", border:"1px solid #1a1a2a", padding:10, marginBottom:12 }}>
            <div style={{ fontSize:9, color:"#444", letterSpacing:3, marginBottom:6 }}>ENVELOPE</div>
            <WaveformViz params={params} />
          </div>

          {/* Wave selector */}
          <div style={{ marginBottom:12 }}>
            <div style={{ fontSize:9, color:"#444", letterSpacing:3, marginBottom:6 }}>WAVEFORM</div>
            <div style={{ display:"flex", gap:4 }}>
              {["sine","square","sawtooth","noise"].map(w => (
                <button key={w} onClick={() => updateParam("wave", w)}
                  style={{
                    padding:"4px 10px", flex:1,
                    background: params.wave===w ? "#1a0a00" : "transparent",
                    border:"1px solid "+(params.wave===w ? "#ff6a00" : "#1a1a2a"),
                    color: params.wave===w ? "#ff6a00" : "#444",
                    cursor:"pointer", fontFamily:"inherit", fontSize:9, letterSpacing:1,
                  }}>
                  {w === "sawtooth" ? "saw" : w}
                </button>
              ))}
            </div>
          </div>

          {/* Sliders */}
          <div style={{ background:"#0a0a14", border:"1px solid #1a1a2a", padding:12, display:"flex", flexDirection:"column", gap:10 }}>
            <Knob label="FREQ" value={params.freq} min={20} max={1200} step={1}
              onChange={v => updateParam("freq", v)} format={v => `${v}Hz`} />
            <Knob label="ATTACK" value={params.attack} min={0.001} max={0.5} step={0.001}
              onChange={v => updateParam("attack", v)} color="#00c6ff" format={v => `${v.toFixed(3)}s`} />
            <Knob label="DECAY" value={params.decay} min={0.01} max={1.0} step={0.01}
              onChange={v => updateParam("decay", v)} color="#00c6ff" format={v => `${v.toFixed(2)}s`} />
            <Knob label="SUSTAIN" value={params.sustain} min={0} max={1} step={0.01}
              onChange={v => updateParam("sustain", v)} color="#00ff88" format={v => `${(v*100).toFixed(0)}%`} />
            <Knob label="RELEASE" value={params.release} min={0.01} max={1.0} step={0.01}
              onChange={v => updateParam("release", v)} color="#00c6ff" format={v => `${v.toFixed(2)}s`} />
            <Knob label="PITCH SWEEP" value={params.pitch_sweep} min={-1} max={1} step={0.01}
              onChange={v => updateParam("pitch_sweep", v)} color="#ff6a00" format={v => v.toFixed(2)} />
            <Knob label="VOLUME" value={params.volume} min={0} max={1} step={0.01}
              onChange={v => updateParam("volume", v)} color="#ffcc00" format={v => `${(v*100).toFixed(0)}%`} />
          </div>

          {/* Play button */}
          <button onClick={play}
            style={{
              width:"100%", marginTop:12, padding:"14px 0",
              background: playing
                ? "linear-gradient(90deg,#00ff88,#00c6ff)"
                : "linear-gradient(90deg,#ff6a00,#ee0979)",
              border:"none", color:"#fff", cursor:"pointer",
              fontFamily:"inherit", fontSize:13, letterSpacing:4,
              fontWeight:"bold", textTransform:"uppercase",
              transition:"all 0.1s",
              boxShadow: playing ? "0 0 20px rgba(0,255,136,0.4)" : "0 0 20px rgba(255,106,0,0.3)",
            }}>
            {playing ? "▶ PLAYING..." : "▶ PLAY"}
          </button>
        </div>

        {/* RIGHT — C code */}
        <div style={{ minWidth:260, flex:1 }}>
          <div style={{ display:"flex", justifyContent:"space-between", alignItems:"center", marginBottom:6 }}>
            <div style={{ fontSize:9, color:"#444", letterSpacing:3 }}>EXPORT → C STRUCT</div>
            <button onClick={() => setShowCode(s => !s)}
              style={{
                padding:"3px 10px", background:"transparent",
                border:"1px solid #1a1a2a", color:"#555",
                cursor:"pointer", fontFamily:"inherit", fontSize:9, letterSpacing:1,
              }}>
              {showCode ? "OCULTAR" : "MOSTRAR"}
            </button>
          </div>

          {showCode && (
            <div style={{ background:"#0a0a14", border:"1px solid #1a1a2a", padding:14 }}>
              <pre style={{
                margin:0, fontSize:10, color:"#00ff88",
                lineHeight:1.7, whiteSpace:"pre-wrap", overflowX:"auto",
              }}>
                {cCode}
              </pre>
            </div>
          )}

          {/* Python equivalent */}
          <div style={{ marginTop:12 }}>
            <div style={{ fontSize:9, color:"#444", letterSpacing:3, marginBottom:6 }}>PYTHON · TERMUX / COLAB</div>
            <div style={{ background:"#0a0a14", border:"1px solid #1a1a2a", padding:14 }}>
              <pre style={{
                margin:0, fontSize:10, color:"#4af",
                lineHeight:1.7, whiteSpace:"pre-wrap",
              }}>
{`import numpy as np
from scipy.io import wavfile

SR = 44100
freq   = ${params.freq.toFixed(1)}
attack = ${params.attack.toFixed(3)}
decay  = ${params.decay.toFixed(3)}
sustain= ${params.sustain.toFixed(2)}
release= ${params.release.toFixed(3)}
sweep  = ${params.pitch_sweep.toFixed(2)}
vol    = ${params.volume.toFixed(2)}

dur = attack+decay+0.15+release
t = np.linspace(0, dur, int(SR*dur))

# Envelope ADSR
env = np.zeros_like(t)
a = int(attack*SR)
d = int((attack+decay)*SR)
s = int((attack+decay+0.15)*SR)
env[:a] = np.linspace(0,vol,a)
env[a:d]= np.linspace(vol,vol*sustain,d-a)
env[d:s]= vol*sustain
env[s:] = np.linspace(vol*sustain,0,len(t)-s)

# Frecuencia con sweep
f = freq * (2**(sweep*2*t/dur))
${params.wave === "noise"
  ? "sig = np.random.randn(len(t))  # noise\nsig *= env"
  : `sig = np.sin(2*np.pi*np.cumsum(f)/SR)\nsig *= env`}

wavfile.write("sfx_${entity}_${params.id}.wav",
              SR, (sig*32767).astype(np.int16))`}
              </pre>
            </div>
          </div>

          {/* All SFX list for entity */}
          <div style={{ marginTop:12, background:"#0a0a14", border:"1px solid #1a1a2a", padding:12 }}>
            <div style={{ fontSize:9, color:"#444", letterSpacing:3, marginBottom:8 }}>
              TODOS LOS SFX — {entity.toUpperCase()}
            </div>
            {sfxList.map((s,i) => (
              <div key={s.id} style={{
                display:"flex", justifyContent:"space-between", alignItems:"center",
                padding:"5px 0", borderBottom:"1px solid #111",
                color: i===sfxIdx ? currentEntity.color : "#444",
                fontSize:10, letterSpacing:1,
              }}>
                <span>SFX_{entity.toUpperCase()}_{s.id.toUpperCase()}</span>
                <span style={{ fontSize:9, color:"#333" }}>{s.wave} · {s.freq}Hz</span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}
