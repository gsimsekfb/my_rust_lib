Your **Surface Pro 7 (16GB RAM / 1TB SSD)** is a highly capable machine for systems programming, but its lack of a dedicated NVIDIA GPU and specialized networking hardware creates clear boundaries between what you can master at home versus what requires a professional lab or cloud environment.

### 2026 Skills Feasibility Table

| Industry Sector | Feasibility on Surface Pro 7 | Barriers / Requirements |
| :--- | :--- | :--- |
| **Non-Web3 Backend** | **Optimal** (10/10) | None. Perfect for Rust/C++ async, gRPC, and distributed systems logic. |
| **Blockchain / Web3** | **High** (9/10) | Node syncing requires high SSD bandwidth (you have this). 16GB RAM is enough for 1–2 local testnets. |
| **Cybersecurity** | **High** (8/10) | Requires **WSL2** or a dedicated **Ubuntu partition** for eBPF/Kernel work. |
| **Database Internals** | **Moderate** (7/10) | Great for logic/coding; difficult to do high-scale performance benchmarking due to thermal throttling. |
| **AI Infrastructure** | **Low** (3/10) | **Missing NVIDIA GPU.** You cannot run CUDA or Triton kernels locally. Requires cloud (Colab/Lambda Labs). |
| **HFT & Fintech** | **Low** (2/10) | **Hardware Gap.** Requires FPGA boards (e.g., Alveo) and 10/25GbE NICs for real-world low-latency simulation. |

---

### 🟢 Easy to Learn at Home (Surface Pro 7 is perfect)

#### 1. Non-Web3 Backend & Systems Architecture
Since you already have **WSL and Ubuntu** installed, you can fully master the "Modern Backend" stack. 
* **What to do:** Focus on **Rust `tokio`**, **io_uring**, and **zero-copy networking**. 
* **Why it works:** These are purely software-bound skills. Your 1TB SSD is great for handling large trace logs during debugging.

#### 2. Cybersecurity (eBPF & Kernel Dev)
eBPF is the "it" skill for 2026 security roles.
* **What to do:** Use your Ubuntu setup to write eBPF programs for network filtering or system-call monitoring.
* **Why it works:** You don't need a supercomputer to write kernel-level probes; you just need a modern Linux kernel, which your Surface handles easily via WSL2.

#### 3. Blockchain (Protocol Development)
* **What to do:** Build your own Layer-2 or a libp2p-based node.
* **Why it works:** While running a *mainnet* Ethereum node is heavy, **development** and **local unit testing** of smart contracts (Rust/Solana) or consensus logic is lightweight.

---

### 🔴 Difficult to Learn at Home (Hardware Limited)

#### 1. AI Infrastructure (The "CUDA" Problem)
Surface Pro 7 uses an **Intel Iris Plus** (integrated) GPU. You cannot run CUDA, which is the industry standard for $200k+ AI roles.
* **The Workaround:** You can write the **Rust orchestration layer** (the "wrapper" that talks to the GPU), but for the actual kernel work, you must rent a GPU in the cloud ($0.50/hr on Lambda Labs or RunPod).

#### 2. HFT (High-Frequency Trading)
HFT is about more than just C++ code; it’s about how that code interacts with **NICs (Network Interface Cards)** and **FPGAs**.
* **The Barrier:** You cannot simulate "microsecond jitter" or "kernel bypass" effectively on a Surface Pro wireless/USB-C network stack. 
* **The Workaround:** Focus on the **Order Matching Engine** logic (pure C++/Rust algorithmics) which can be done on any machine.

---

### Summary Recommendation
To hit that **$200k+ remote goal** with your current hardware, your "Golden Path" is **Non-Web3 Backend (Systems/Infrastructure)** or **Cybersecurity**. 

> **Action Plan:** Spend your time on your **Ubuntu/WSL** environment mastering **Rust + Linux Internals (eBPF/io_uring)**. This combination is currently the highest-ROI skill set that doesn't require a $5,000 workstation or specialized hardware.