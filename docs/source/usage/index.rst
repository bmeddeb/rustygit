Usage Guide
===========

This section provides detailed information on how to use RustyGit for common Git operations.

.. toctree::
   :maxdepth: 2

   repositories
   commits
   branches

Basic Usage
----------

Here's a quick overview of how to use RustyGit:

.. code-block:: python

   import rustygit

   # Open an existing repository
   repo = rustygit.Repository("/path/to/repo")

   # Check if it's a bare repository
   is_bare = repo.is_bare()

   # Initialize a new repository
   new_repo = rustygit.Repository.init("/path/to/new/repo")

   # Clone a repository
   cloned_repo = rustygit.Repository.clone("https://github.com/user/repo.git", "/path/to/clone")

The following pages in this section will go into more detail about specific use cases and operations.