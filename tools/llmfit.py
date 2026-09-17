"""llmfit — bu makinede hangi LLM/modeller sığar? SafetyNet için.
Kullanim: python tools/llmfit.py
Ollama yoksa bile RAM/VRAM'e göre önerir. nvidia-smi varsa VRAM'i oradan okur.
"""
import shutil, subprocess, platform

def vram_mib():
    if shutil.which("nvidia-smi"):
        try:
            out = subprocess.check_output(
                ["nvidia-smi", "--query-gpu=memory.total", "--format=csv,noheader,nounits"],
                text=True, timeout=10)
            return int(out.strip().splitlines()[0])
        except Exception:
            pass
    return 0

def ram_gb():
    try:
        import ctypes
        class MS(ctypes.Structure):
            _fields_ = [("dwLength", ctypes.c_ulong), ("dwMemoryLoad", ctypes.c_ulong),
                        ("ullTotalPhys", ctypes.c_ulonglong), ("ullAvailPhys", ctypes.c_ulonglong),
                        ("ullTotalPageFile", ctypes.c_ulonglong), ("ullAvailPageFile", ctypes.c_ulonglong),
                        ("ullTotalVirtual", ctypes.c_ulonglong), ("ullAvailVirtual", ctypes.c_ulonglong),
                        ("ullAvailExtendedVirtual", ctypes.c_ulonglong)]
        s = MS(); s.dwLength = ctypes.sizeof(MS)
        ctypes.windll.kernel32.GlobalMemoryStatusEx(ctypes.byref(s))
        return round(s.ullTotalPhys / (1024**3), 1)
    except Exception:
        return 0.0

MODELS = [
    # (ad, gereken VRAM MiB Q4, gereken RAM GB CPU-only, not)
    ("qwen2.5:0.5b-instruct", 600, 2, "kapı cümlesi + etiket, eski laptop bile koşturur"),
    ("qwen2.5:1.5b-instruct", 1200, 3, "Türkçe iyi, 960M'de sınırda, 3060'da rahat"),
    ("qwen2.5:3b-instruct", 2200, 5, "özet + cross-ref yorumu için ideal"),
    ("llama3.2:3b-instruct", 2200, 5, "alternatif, Türkçe Qwen kadar iyi değil"),
    ("qwen2.5:7b-instruct-q4_K_M", 4800, 9, "gece özetleri + olay analizi, bu makinede olur"),
    ("qwen2.5:14b-instruct-q4_K_M", 9000, 17, "bu makineye sığmaz (6GB VRAM), CPU'da çok yavaş"),
]

def main():
    vram = vram_mib()
    ram = ram_gb()
    print(f"OS: {platform.system()} {platform.machine()} | CPU: {platform.processor() or 'n/a'}")
    print(f"RAM: {ram} GB | VRAM: {vram} MiB ({vram/1024:.1f} GB)")
    print("\nModel sığma tablosu (Q4 quant):")
    for name, need_vram, need_ram, note in MODELS:
        gpu_ok = "EVET" if vram >= need_vram else "hayır"
        cpu_ok = "EVET" if ram >= need_ram else "hayır"
        fit = "[OK]" if (gpu_ok == "EVET" or cpu_ok == "EVET") else "[YOK]"
        print(f"  {fit:6s} {name:32s} GPU:{gpu_ok:5s} ({need_vram}MiB) CPU:{cpu_ok:5s} ({need_ram}GB) - {note}")
    print("\nSafetyNet önerisi:")
    if vram >= 5000:
        print("  - Bu makine = BEYİN: 7B'ye kadar LLM + ağır TTS + gece cross-ref analizi burada.")
        print("  - Memleket laptopu = NÖBETÇİ: 0.5B + YOLO + yüz, kapı kararını o verir (internet kesilse de çalışır).")
    elif vram > 0:
        print("  - GPU küçük: 3B'ye kadar GPU, üstü CPU.")
    else:
        print("  - GPU yok: 0.5B-1.5B CPU ile, kısa cümleler.")

if __name__ == "__main__":
    main()
