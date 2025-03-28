API Reference
============

This section provides detailed documentation for the RustyGit API.

.. toctree::
   :maxdepth: 2

   repository
   commit
   branch

Overview
-------

RustyGit provides several main classes:

* :class:`Repository` - Represents a Git repository
* :class:`Commit` - Represents a Git commit
* :class:`Branch` - Represents a Git branch

Example Usage
-----------

.. code-block:: python

   import rustygit

   # Open a repository
   repo = rustygit.Repository("/path/to/repo")

   # Initialize a new repository
   new_repo = rustygit.Repository.init("/path/to/new/repo")

   # Clone a repository
   cloned_repo = rustygit.Repository.clone("https://github.com/user/repo.git", "/path/to/clone")

   # Create and use Commit object
   commit = rustygit.Commit(
       id="abcdef1234567890",
       message="Example commit message",
       author_name="John Doe",
       author_email="john@example.com"
   )

   # Create and use Branch object
   branch = rustygit.Branch("main", False)  # local branch
   remote_branch = rustygit.Branch("origin/main", True)  # remote branch