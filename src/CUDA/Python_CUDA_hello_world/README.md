# 🐍 Using CUDA with Python in WSL2

## 📦 1. Install Python + Numba

```bash
sudo apt install python3-pip
pip3 install numba numpy
```


Note: you could get a PEP 668 situation if your distro is a Debian-based systems (like Ubuntu in WSL2), which are now protecting their system Python from being altered by pip directly — that’s why your could be seeing an externally-managed-environment error when trying to install numba.
To fix your you can create a virtual environment:
First, make sure the virtual environment tools are installed:

```bash
sudo apt install python3-venv python3-full
```
Create a virtual environment:

```bash
python3 -m venv ~/cuda-env
```

```bash
source ~/cuda-env/bin/activate
```

And then install [numba](https://numba.pydata.org/), numpy, and friends:

```bash
pip install numba numpy
```

💥 Boom! You’re in a safe zone now. You can run your GPU Python code inside this environment without touching your system Python.

Once you’re done, you can deactivate the virtual environment with:
```bash
deactivate
```

Try this Python+CUDA code:

```python
from numba import cuda
import numpy as np

@cuda.jit
def multiply_by_two(arr):
    i = cuda.grid(1)
    if i < arr.size:
        arr[i] *= 2

data = np.arange(10)
d = cuda.to_device(data)
multiply_by_two[1, 10](d)
print(d.copy_to_host())
```

---

### 🧪 Verify GPU Access in Python

```python
from numba import cuda
print(cuda.detect())
```

It should list your GTX 1660!

---

### 🔧 Pro Tips

- Always run `nvidia-smi` in both **Windows** and **WSL2** to debug GPU visibility.
- You can use **VSCode + WSL Remote** to write CUDA code in Linux and compile with `nvcc` directly from Windows.

---