Commit
======

.. py:class:: rustygit.Commit(id, message=None, author_name=None, author_email=None, time=0)

   Represents a Git commit.

   :param str id: The commit hash.
   :param str message: The commit message.
   :param str author_name: Name of the author.
   :param str author_email: Email of the author.
   :param int time: Timestamp (Unix time).

   .. py:attribute:: id
      :type: str

      The commit hash.

   .. py:attribute:: message
      :type: str or None

      The commit message.

   .. py:attribute:: author_name
      :type: str or None

      Name of the author.

   .. py:attribute:: author_email
      :type: str or None

      Email of the author.

   .. py:attribute:: time
      :type: int

      Timestamp (Unix time).

Examples
--------

Create a Commit object:

.. code-block:: python

   import rustygit

   commit = rustygit.Commit(
       id="abcdef1234567890",
       message="Fix typo in README",
       author_name="John Doe",
       author_email="john@example.com",
       time=1622548800
   )

Access commit properties:

.. code-block:: python

   import rustygit

   commit = rustygit.Commit(id="abcdef1234567890", message="Example commit")

   print(f"Commit ID: {commit.id}")
   print(f"Message: {commit.message}")

   if commit.author_name:
       print(f"Author: {commit.author_name}")

   if commit.author_email:
       print(f"Email: {commit.author_email}")

   print(f"Timestamp: {commit.time}")