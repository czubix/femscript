# Femscript

Femscript is a simple scripting language created as a personal project to learn Rust. It is asynchronous, allowing you to execute Femscript code in Python, exchange Python values with Femscript, and vice versa, all with seamless integration.

## Features

- **Asynchronous:** Femscript is designed to be asynchronous, ensuring it does not block the execution of the main code.
- **Easy Integration with Python:** Executing Femscript code in Python is straightforward, and binding functions and passing variables between Python and Femscript is simple.
- **Lightweight:** The project is lightweight and easy to set up, making it suitable for quick prototyping and integration into existing projects.

## Installation

To install Femscript, follow the steps below:

### Requirements

- You need to have **Rust** and **Make** installed on your system.
- Install the required Python packages:
    ```bash
    pip install -r requirements.txt
    ```

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

## Usage

Here is an example of how to use Femscript in your Python code:

```py
import asyncio
from femscript import Femscript, var

async def main() -> None:
    fs = Femscript("x = 35; x + y", variables=[var("y", 34)])
    result = await fs.execute()
    print(result) # Outputs: 69

if __name__ == "__main__":
    asyncio.run(main())
```

## Docs

The documentation is currently minimal but will be expanded over time. What exists now can be found [here](https://czubix.github.io/femscript)

## Example Uses

Here are a few example use cases for Femscript:

- **Adding Custom Commands in a Discord Bot:**
  Femscript can be used to allow users to write their own custom commands for a Discord bot. With the ability to easily bind functions and pass variables between Python and Femscript, you can integrate scripting functionality directly into your bot.

- **Feel free to come up with your own use cases:**
  Since Femscript is flexible, you can think of many different ways to use it. The possibilities are yours to explore!

## What Uses Femscript

- **~~Cenzura~~ Fembot Discord Bot:**
  Femscript is used in the [Fembot Discord Bot](https://github.com/poligonteam/cenzura) to allow dynamic scripting and custom command handling.

- **Femscript Playground:**
  You can try Femscript on my website at [czubix.dev](https://czubix.dev), where the **Femscript Playground** allows you to experiment with the language.

## License

Femscript is licensed under the Apache 2.0 License. See the LICENSE file for more details.

> *This README was written with the help of ChatGPT, because I’m too dumb to do it on my own.*