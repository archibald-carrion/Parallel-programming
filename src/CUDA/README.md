# 🐧🚀 Setting Up CUDA in WSL2

## ✅ Prerequisites

1. **Windows 10** or **Windows 11**
2. **WSL2 installed and running Ubuntu (or another supported distro)**
3. **NVIDIA driver with WSL2 support installed on Windows**
4. **A CUDA compatible GPU** In my case I am using a GTX 1660 (CUDA Compute Capability 7.5 — fully supported!)

---

### 🧱 Step-by-Step Setup

---

#### 🔹 1. **Install WSL2 (if not already)**

```powershell
wsl --install
```

Choose Ubuntu when prompted or use:
```powershell
wsl --install -d Ubuntu
```

---

#### 🔹 2. **Update NVIDIA GPU Driver (Very Important)**

Download the latest driver that supports **CUDA on WSL**:
👉 [https://developer.nvidia.com/cuda/wsl](https://developer.nvidia.com/cuda/wsl)

Make sure `nvidia-smi` works in **Windows PowerShell** before continuing.

---

#### 🔹 3. **Install CUDA Toolkit in WSL2**

Open your Ubuntu shell and run:

```bash
sudo apt update
sudo apt install build-essential dkms
sudo apt install nvidia-cuda-toolkit
```

**Check CUDA installation:**
```bash
nvcc --version
nvidia-smi
```

If `nvidia-smi` works — you're good to go! :rocket:

---

#### 🔹 4. **Run a CUDA Sample**

```bash
mkdir cuda-test
cd cuda-test
vim main.cu
```

Paste this sample:

```cpp
#include <stdio.h>

// host code 
void CPUFunction()
{
  printf("Hello from the CPU.\n");
}

// device code
__global__ void GPUFunction()
{
  printf("Hello from the GPU.\n");
}

int main()
{
  CPUFunction();

  GPUFunction<<<1, 1>>>();
  cudaDeviceSynchronize();
}
```

Compile and run:
```bash
nvcc main.cu -o main
./main
```

You should see this output:
```
Hello from the CPU.
Hello from the GPU.
```
