# Installation

To install Femscript, follow the steps below:

---

### Requirements

- You need to have **Rust** and **Make** installed on your system.
- Install the required Python packages:
    ```bash
    pip install -r requirements.txt
    ```

---

### Installation Steps

1. Clone the repository:
    ```bash
    git clone https://github.com/czubix/femscript.git
    ```

2. Navigate to the project directory:
    ```bash
    cd femscript
    ```

3. Build the project:
    ```bash
    make build PYTHON_VERSION=your_python_command
    ```
    (Optional: To build in release mode, use the following command instead):
    ```bash
    make build PYTHON_VERSION=your_python_command EXTRA_ARGS="--release"
    ```

4. Install the package:
    ```bash
    make install PYTHON_VERSION=your_python_command PIP_VERSION=your_pip_command
    ```