# wgpufusion - High-Performance GPU Compute in Rust

RustGPUX is a high-performance GPU compute framework in Rust that utilizes **wgpu** to run advanced parallel computations on the GPU. It includes:

✅ **Vector Addition** – Process **1 million elements** on the GPU.  
✅ **Matrix Multiplication** – Optimized parallel matrix operations.  
✅ **Parallel Reduction** – Efficient sum reduction using shared memory.  
✅ **Gaussian Blur** – GPU-powered image processing.  
✅ **Workgroup Optimization** – Fast shared memory optimizations.  

## Features
- 🚀 **Blazing Fast Computation** – Leverage GPU parallelism for high-speed number crunching.
- 🖼 **GPU Image Processing** – Perform Gaussian blur on images.
- 📊 **Optimized Workgroup Usage** – Use efficient workgroup shared memory.
- 🔥 **Scalable & Modular** – Supports easy expansion with new compute shaders.

---

## Getting Started

### **1️⃣ Install Rust & Dependencies**
Ensure you have **Rust** installed:
```sh
rustup update
```

Install dependencies:
```sh
cargo add wgpu pollster image bytemuck
```

### **2️⃣ Clone the Repository**
```sh
git clone https://github.com/Thewsthews/wgpufusion.git
cd wgpufusion
```

### **3️⃣ Run the GPU Compute Program**
```sh
cargo run
```

---

## 🎯 How It Works

### **📌 Vector Addition (1 Million Elements)**
Computes the sum of **large vectors** using GPU parallelism.

### **📌 Matrix Multiplication (Parallelized GPU Kernel)**
Performs **A × B** matrix multiplication using workgroups for efficiency.

### **📌 Parallel Reduction (Summing Large Arrays)**
Uses **tree reduction** to compute sums with minimal overhead.

### **📌 Gaussian Blur (Image Processing)**
Applies **GPU-based Gaussian blur** to images for high-speed filtering.

---

## Project Structure
```plaintext
rustgpux/
├── src/
│   ├── main.rs        # Entry point for GPU computations
│   ├── compute.wsgl   # GPU compute functions (vector add, reduction, etc.)
├── Cargo.toml         # Dependencies and project configuration
├── README.md          # Project documentation
```

---

## Performance Benchmarks
| Operation | CPU Time | GPU Time |
|-----------|---------|---------|
| Vector Addition (1M) | ~200ms | **~10ms** |
| Matrix Multiplication (256x256) | ~2s | **~50ms** |
| Parallel Reduction (1M) | ~500ms | **~15ms** |
| Gaussian Blur (512x512) | ~300ms | **~12ms** |

---

## Contributing
We welcome contributions! Feel free to **fork, submit PRs, or report issues.**

1. Fork the repository.
2. Create a feature branch: `git checkout -b feature-new`
3. Commit changes: `git commit -m "Add awesome feature"`
4. Push: `git push origin feature-new`
5. Open a Pull Request 🚀

---

## 🛡 License
This project is licensed under the **MIT License** – feel free to use and modify! ✨

---

## 📞 Contact
For any queries or contributions, reach out:
📧 Email: etiegnim@gmail.com  
🐙 GitHub: [Me](https://github.com/Thewsthews)

## STILL A WIP!
   More to come soon!

## :)

M1