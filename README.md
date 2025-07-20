Nostalgia RBucklespring keyboard sound now in Rust!
=====================================

Copyright 2016 Ico Doornekamp

This project emulates the sound of my old faithful IBM Model-M space saver
bucklespring keyboard while typing on my notebook, mainly for the purpose of
annoying the hell out of my coworkers.

> Simulates the IBM Model M experience — clicky, loud, and pure 80s typing nostalgia.

![Model M](img/model-m.jpg)
![Buckle](img/buckle.gif)

RBucklespring runs as a background process and plays back the sound of each key
pressed and released on your keyboard, just as if you were using an IBM
Model-M. The sound of each key has carefully been sampled, and is played back
while simulating the proper distance and direction for a realistic 3D sound
palette of pure nostalgic bliss.



### 🛠️ Build from source (Linux)

#### 🔧 Dependencies

Debian/Ubuntu:

```bash
sudo apt install libasound2-dev libudev-dev
```

Arch Linux:

```bash
sudo pacman -S alsa-lib libudev
```

Fedora:

```bash
sudo dnf install alsa-lib-devel systemd-devel
```

#### 🛠 Build & run

```bash
cargo build --release
./target/release/RBucklespring
```

### 🪟 Windows Support

**It works.** Just run:

```powershell
cargo run --release
```

### 🪟 Windows Support

**It works.** Just run:

```powershell
cargo run --release
```

Ensure `.wav` files are in the `wav/` folder and named like `1e-0.wav`, `1e-1.wav`, etc.

---


## ❓ FAQ

**Q: Does this support Wayland?**
Yes/No! `rdev` captures input via evdev under `/dev/input`, which works under Wayland too. Still not sure xD

**Q: Does it require root?**
⚠️ Sometimes yes, unless you configure a udev rule to allow reading from `/dev/input/event*`.

**Q: Can I run this as a background service?**
✅ Yes, or add it to autostart via `.desktop` on Linux or Task Scheduler on Windows.

**Q: Why is audio failing with ALSA errors?**
🔊 Switch to PulseAudio or PipeWire. Or set a proper `~/.asoundrc` file.

---

## ❤️ Credits

* Based on the original [Bucklespring](https://github.com/zevv/bucklespring) by [Ico Doornekamp](https://github.com/zevv)
