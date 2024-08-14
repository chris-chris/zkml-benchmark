# o1js Example

## Introduction

This project demonstrates the implementation of both Linear Regression and a Multi-Layer Perceptron (MLP) using the `o1js` library.
The Linear Regression model serves as a fundamental building block, while the MLP is designed to take five input values and predict an output through two hidden layers.
Additionally, we provide a Jupyter Notebook implemented in PyTorch that mimics the structure of the MLP defined in `o1js`.

## How to Run

### Environment

- Node.js (`v20.11.1`)
- Python 3.x (for the Jupyter Notebook)

### Steps to Execute

1. **Build the project**:
   This step compiles TypeScript code (`mlp.ts`) into JavaScript (`mlp.js`).

   ```bash
   npm run build
   ```

2. **Run the project**:
   Execute the compiled JavaScript file.

   ```bash
   node dist/mlp.js
   ```

3. **Benchmarking**:
   To benchmark the performance of the Linear Regression and MLP implementations, you can use `gtime` to measure execution time.

   Run the script to install `gtime`:

   ```bash
   sh install_gtime.sh
   ```

   **Run the benchmarks**: After installing `gtime`, you can benchmark the two models by running the following commands:

   ```bash
   sh bench.sh
   ```

4. **Jupyter Notebook**:
   The project includes a Jupyter Notebook (`src/notebook/mlp_comparison.ipynb`) that implements the same MLP model in PyTorch as defined in the `o1js` TypeScript code.
   This notebook allows you to compare the outputs of both models.

   To run the notebook:

   1. Navigate to the `models/mlp` directory.

      ```bash
      cd models/mlp
      ```

   2. Open the notebook using Jupyter:

      ```bash
      jupyter notebook ./mlp.ipynb
      ```

   3. Execute the cells to see the PyTorch implementation of the MLP and compare it with the `o1js` version.

## MLP Implementation

### Overview

The MLP (Multi-Layer Perceptron) implemented in this project consists of the following components:

- **Input Layer**: Takes 5 input values.
- **Hidden Layer 1**: Applies linear transformation followed by a ReLU activation function.
- **Hidden Layer 2**: Takes the output of the first hidden layer, applies another linear transformation, and passes it through another ReLU activation.
- **Output Layer**: Produces the final output value using a linear transformation.

### Diagram of MLP Structure

To better understand the MLP structure, refer to the following diagram, which represents the data flow and operations within the network:

```
Input Layer (5 inputs)
  |
  v
+------------------------+
|  Hidden Layer 1        |  (Linear Regression + ReLU)
|  Weighted Sum (z1)     | -> ReLU(z1) -> a1
+------------------------+
  |
  v
+------------------------+
|  Hidden Layer 2        |  (Linear Regression + ReLU)
|  Weighted Sum (z2)     | -> ReLU(z2) -> a2
+------------------------+
  |
  v
+------------------------+
|  Output Layer          |  (Linear Regression)
|  Weighted Sum (z3)     | -> Output (z3)
+------------------------+
  |
  v
Output
```

## Benchmark Result

```bash
model,exp_num,proving_time,memory_usage,cpu_usage
ezkl,1,67.66,1344276,757%
o1js,1,165.54,1090636,841%
ezkl,2,70.15,1343040,748%
o1js,2,208.12,1168216,864%
ezkl,3,69.18,1360868,688%
o1js,3,201.29,1166896,849%
ezkl,4,69.08,1319804,711%
o1js,4,198.60,1147124,862%
ezkl,5,68.62,1355372,770%
o1js,5,223.88,1216168,881%
```
