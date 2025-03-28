Working with Repositories
======================

This page covers operations related to Git repositories.

Opening a Repository
------------------

To open an existing Git repository:

.. code-block:: python

   import rustygit

   # Open an existing repository
   repo = rustygit.Repository("/path/to/repo")

Creating a New Repository
-----------------------

To initialize a new Git repository:

.. code-block:: python

   import rustygit

   # Initialize a new repository
   repo = rustygit.Repository.init("/path/to/new/repo")

Cloning a Repository
------------------

To clone a repository from a URL:

.. code-block:: python

   import rustygit

   # Clone a repository
   repo = rustygit.Repository.clone("https://github.com/user/repo.git", "/path/to/clone")

Repository Information
--------------------

To check if a repository is bare (has no working directory):

.. code-block:: python

   import rustygit

   repo = rustygit.Repository("/path/to/repo")

   if repo.is_bare():
       print("This is a bare repository")
   else:
       print("This is a regular repository with a working directory")

Error Handling
------------

RustyGit uses Python exceptions to handle errors. When operating on repositories, you might encounter:

.. code-block:: python

   import rustygit

   try:
       repo = rustygit.Repository("/path/to/nonexistent/repo")
   except IOError as e:
       print(f"Could not open repository: {e}")

   try:
       repo = rustygit.Repository.clone("invalid://url", "/path/to/clone")
   except IOError as e:
       print(f"Could not clone repository: {e}")