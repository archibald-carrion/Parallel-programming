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