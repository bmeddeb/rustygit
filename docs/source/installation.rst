Installation
============

Requirements
-----------

* Python 3.7 or higher
* pip (Python package installer)

Installing from PyPI
------------------

The easiest way to install RustyGit is via pip:

.. code-block:: bash

   pip install rustygit

This will download and install the pre-compiled wheels for your platform. RustyGit provides wheels for:

* Windows (32/64-bit)
* macOS (Intel and Apple Silicon)
* Linux (manylinux compatible)

Installing from Source
--------------------

To install from source, you'll need:

* Rust compiler (1.63 or higher)
* Cargo (Rust package manager)
* Python development headers
* C compiler

1. Clone the repository:

   .. code-block:: bash

      git clone https://github.com/yourusername/rustygit.git
      cd rustygit

2. Install with pip in development mode:

   .. code-block:: bash

      pip install -e .

   This will build the Rust extension and install it in development mode.

Verifying Installation
--------------------

To verify that RustyGit is installed correctly, run:

.. code-block:: python

   >>> import rustygit
   >>> rustygit.__version__
   '0.1.0'