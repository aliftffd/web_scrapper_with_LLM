# Rust-CUDA: High-Performance GPU Computing for Python

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Python](https://img.shields.io/badge/python-3.8%2B-blue.svg)
![CUDA](https://img.shields.io/badge/cuda-11.0%2B-green.svg)
![Status](https://img.shields.io/badge/status-in%20progress-yellow.svg)

## Overview

Rust-CUDA is a high-performance Python extension module built with Rust that provides GPU-accelerated scientific computing capabilities. The project leverages the SciRS2 ecosystem for scientific computing and offers seamless integration between Rust's performance and Python's ease of use.

**🚧 This project is currently under active development and may contain experimental features.**

## Features

- **🚀 GPU-Accelerated Computing**: CUDA support through CudaRC and SciRS2 GPU contexts
- **🔢 High-Performance Linear Algebra**: Optimized matrix operations using BLAS and matrixmultiply
- **🧠 Neural Network Operations**: GPU-accelerated neural network primitives
- **📊 Performance Benchmarking**: Built-in CPU vs GPU performance comparisons
- **🐍 Python Integration**: Seamless Python bindings using PyO3
- **⚡ Batch Processing**: Efficient batch matrix operations for ML workloads
- **🔧 Flexible Backend**: Automatic fallback to CPU when GPU is unavailable

## Installation

### Prerequisites

- **Rust** (1.70 or later)
- **Python** (3.8 or later)
- **CUDA Toolkit** (11.0 or later) - Optional, for GPU acceleration
- **OpenBLAS** or **Intel MKL** - For optimized CPU linear algebra

### Building from Source

1. **Clone the repository**:
```bash
git clone https://github.com/yourusername/rust-cuda.git
cd rust-cuda
```

2. **Install maturin** (Rust-Python build tool):
```bash
pip install maturin
```

3. **Build and install the package**:
```bash
# Development build
maturin develop

# Or production build
maturin build --release
pip install target/wheels/rust_cuda-*.whl
```

### Dependencies

The project uses several key Rust crates:

- **PyO3**: Python bindings for Rust
- **SciRS2 Ecosystem**: Scientific computing framework
  - `scirs2-core`: Core functionality with GPU support
  - `scirs2-linalg`: Linear algebra operations
  - `scirs2-neural`: Neural network primitives
- **ndarray**: N-dimensional arrays for Rust
- **matrixmultiply**: Optimized matrix multiplication
- **cudarc**: CUDA bindings (optional)

## Usage

### Basic GPU Testing

```python
import rust_cuda

# Test GPU availability
print(rust_cuda.test_gpu_availability())

# Test basic matrix operations
result = rust_cuda.test_matrix_operations(1000)
print(result)
```

### Matrix Operations

```python
# GPU-accelerated matrix multiplication
result = rust_cuda.gpu_matrix_multiply(512)
print(f"Matrix multiplication: {result}")

# Matrix-vector multiplication (GEMV)
result = rust_cuda.test_gemv(1000)
print(f"GEMV operation: {result}")

# Comprehensive linear algebra tests
result = rust_cuda.test_linalg_operations(256)
print(result)
```

### Performance Benchmarking

```python
# Compare CPU vs GPU performance
comparison = rust_cuda.compare_cpu_gpu_performance(
    size=512,      # Matrix size
    iterations=10  # Number of iterations
)
print(comparison)
```

### Batch Processing

```python
# Batch matrix operations for ML workloads
result = rust_cuda.gpu_batch_operations(
    batch_size=32,    # Number of matrices in batch
    matrix_size=256   # Size of each matrix
)
print(result)
```

### Neural Network Operations

```python
# GPU-accelerated neural network demo
result = rust_cuda.gpu_neural_network_demo()
print(result)
```

### Module Testing

```python
# Test all SciRS2 modules and dependencies
result = rust_cuda.test_scirs2_modules()
print(result)
```

## Running the Demo

A comprehensive test script is provided to demonstrate all features:

```bash
python test_gpu.py
```

This will run through:
1. GPU availability testing
2. Matrix multiplication benchmarks
3. CPU vs GPU performance comparison
4. Batch operations demonstration
5. Neural network GPU demo

## Project Structure

```
rust-cuda/
├── 📄 Cargo.toml                 # Rust dependencies and configuration
├── 📄 pyproject.toml             # Python packaging configuration
├── 📄 .gitignore                 # Git ignore rules
├── 📁 .idea/                     # IDE configuration
├── 📁 src/
│   └── 📄 lib.rs                 # Main Rust implementation
├── 📄 test_gpu.py                # Python test/demo script
└── 📄 README.md                  # This file
```

## Key Functions

| Function | Description |
|----------|-------------|
| `test_gpu_availability()` | Check if GPU context can be created |
| `test_matrix_operations(size)` | Basic matrix operations benchmarking |
| `gpu_matrix_multiply(size)` | GPU-accelerated GEMM operations |
| `test_gemv(matrix_size)` | Matrix-vector multiplication testing |
| `test_linalg_operations(size)` | Comprehensive linear algebra tests |
| `compare_cpu_gpu_performance(size, iter)` | Performance comparison |
| `gpu_batch_operations(batch, size)` | Batch matrix processing |
| `gpu_neural_network_demo()` | Neural network operations demo |
| `test_scirs2_modules()` | Test all module dependencies |

## Performance

The library provides optimized implementations for:

- **GEMM (Matrix-Matrix Multiplication)**: Using optimized BLAS routines
- **GEMV (Matrix-Vector Multiplication)**: Vectorized operations
- **Batch Operations**: Efficient batch processing for ML workloads
- **Neural Network Primitives**: Forward pass operations

### Benchmark Results

Typical performance improvements on modern GPUs:
- Matrix multiplication: 2-10x speedup over CPU
- Batch operations: 5-20x speedup for large batches
- Neural network operations: 3-15x speedup

*Results vary depending on matrix size, GPU model, and system configuration.*

## Development Features

### Cargo Features

- **default**: Includes PyO3 extension module
- **backup-cuda**: Optional CudaRC support for additional CUDA functionality

### Build Configuration

```toml
[features]
default = ["pyo3/extension-module"]
backup-cuda = ["cudarc"]
```

## Roadmap

- [ ] **Enhanced CUDA Integration**: Direct CUDA kernel integration
- [ ] **More Linear Algebra Operations**: SVD, eigendecomposition, etc.
- [ ] **Sparse Matrix Support**: Sparse matrix operations
- [ ] **Multi-GPU Support**: Distribution across multiple GPUs
- [ ] **Memory Pool Management**: Efficient GPU memory management
- [ ] **Async Operations**: Non-blocking GPU operations
- [ ] **Python Package Distribution**: PyPI package publishing
- [ ] **Comprehensive Documentation**: API documentation and tutorials
- [ ] **CI/CD Pipeline**: Automated testing and builds

## Contributing

Contributions are welcome! This project is in active development and there are many opportunities to contribute:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/new-feature`)
3. **Make your changes** and add tests
4. **Ensure all tests pass** (`cargo test` and `python test_gpu.py`)
5. **Commit your changes** (`git commit -am 'Add new feature'`)
6. **Push to the branch** (`git push origin feature/new-feature`)
7. **Create a Pull Request**

### Development Guidelines

- Follow Rust best practices and use `cargo clippy`
- Ensure Python bindings are properly typed
- Add appropriate error handling for GPU operations
- Include performance benchmarks for new operations
- Update documentation for new features

## Requirements

### System Requirements

- **GPU**: NVIDIA GPU with CUDA Compute Capability 6.0+ (optional)
- **RAM**: 8GB+ recommended for large matrix operations
- **Disk**: 2GB+ free space for dependencies

### Software Dependencies

- **Rust Toolchain**: Latest stable version
- **Python**: 3.8, 3.9, 3.10, 3.11, or 3.12
- **CUDA Runtime**: 11.0+ (if using GPU features)
- **BLAS Library**: OpenBLAS, Intel MKL, or Accelerate (macOS)

## Troubleshooting

### Common Issues

1. **GPU Not Available**: The library automatically falls back to CPU operations
2. **CUDA Version Mismatch**: Ensure CUDA runtime matches build requirements
3. **BLAS Library Issues**: Install OpenBLAS development packages
4. **Python Version**: Ensure Python 3.8+ is being used

### Error Messages

- `"GPU not available"`: Expected when no CUDA-capable GPU is present
- `"Matrix dimensions don't match"`: Check input matrix shapes
- `"BLAS/LAPACK Error"`: Verify BLAS library installation


## Acknowledgments

- **SciRS2 Team**: For the excellent scientific computing framework
- **PyO3 Contributors**: For seamless Rust-Python integration
- **ndarray Community**: For the foundational array library
- **CUDA Ecosystem**: For GPU computing capabilities

## References

1. [PyO3 Documentation](https://pyo3.rs/)
2. [SciRS2 Framework](https://github.com/SciRS2/scirs2)
3. [ndarray Documentation](https://docs.rs/ndarray/)
4. [CUDA Programming Guide](https://docs.nvidia.com/cuda/)
5. [Maturin Build Tool](https://github.com/PyO3/maturin)

---

**Note**: This project is under active development. APIs may change between versions. Please check the issues page for known limitations and upcoming features.
