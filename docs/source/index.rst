Welcome to RustyGit's documentation!
================================

RustyGit is a high-performance Git operations library written in Rust with Python bindings.
It leverages Rust's speed and memory safety while providing a Pythonic interface for working with Git repositories.

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   introduction
   installation
   usage/index
   examples/index
   api/index
   contributing
   changelog

Features
--------

* High-performance Git operations through Rust and libgit2
* Parallel operations using Rayon
* Async operations with Tokio
* Pythonic API for working with repositories, commits, branches, and more
* Memory safety and better error handling

Installation
-----------

You can install RustyGit using pip:

.. code-block:: bash

   pip install rustygit

Quick Example
------------

.. code-block:: python

   import rustygit

   # Open a repository
   repo = rustygit.Repository("/path/to/repo")

   # Check if it's a bare repository
   is_bare = repo.is_bare()

Indices and tables
=================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`