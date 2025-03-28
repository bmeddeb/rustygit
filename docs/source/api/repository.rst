Repository
==========

.. py:class:: rustygit.Repository(path)

   Represents a Git repository.

   :param str path: Path to the existing repository.
   :raises IOError: If the repository cannot be opened.

   .. py:classmethod:: init(path)

      Initialize a new Git repository at the given path.

      :param str path: Path where the repository should be created.
      :return: A new Repository instance.
      :rtype: Repository
      :raises IOError: If the repository cannot be initialized.

   .. py:classmethod:: clone(url, path)

      Clone a repository from the specified URL to the given path.

      :param str url: URL to clone from.
      :param str path: Path where the repository should be cloned.
      :return: A new Repository instance.
      :rtype: Repository
      :raises IOError: If the repository cannot be cloned.

   .. py:method:: is_bare()

      Check if the repository is bare (has no working directory).

      :return: True if the repository is bare, False otherwise.
      :rtype: bool

Examples
--------

Open an existing repository:

.. code-block:: python

   import rustygit

   repo = rustygit.Repository("/path/to/repo")

Initialize a new repository:

.. code-block:: python

   import rustygit

   new_repo = rustygit.Repository.init("/path/to/new/repo")

Clone a repository:

.. code-block:: python

   import rustygit

   cloned_repo = rustygit.Repository.clone("https://github.com/user/repo.git", "/path/to/clone")

Check if a repository is bare:

.. code-block:: python

   import rustygit

   repo = rustygit.Repository("/path/to/repo")
   if repo.is_bare():
       print("This is a bare repository")