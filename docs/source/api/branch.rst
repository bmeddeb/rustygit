Branch
======

.. py:class:: rustygit.Branch(name, is_remote)

   Represents a Git branch.

   :param str name: The branch name.
   :param bool is_remote: Whether the branch is remote.

   .. py:attribute:: name
      :type: str

      The branch name.

   .. py:attribute:: is_remote
      :type: bool

      Boolean indicating whether the branch is remote.

Examples
--------

Create a Branch object:

.. code-block:: python

   import rustygit

   # Create a local branch
   local_branch = rustygit.Branch("main", False)

   # Create a remote branch
   remote_branch = rustygit.Branch("origin/main", True)

Access branch properties:

.. code-block:: python

   import rustygit

   branch = rustygit.Branch("main", False)

   print(f"Branch name: {branch.name}")
   print(f"Is remote branch: {branch.is_remote}")

   # Determine branch type
   branch_type = "remote" if branch.is_remote else "local"
   print(f"This is a {branch_type} branch")