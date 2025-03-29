class Commit:
    """
    Represents a Git commit, providing detailed metadata about each change in the repository.
    """

    @property
    def hash(self) -> str:
        """SHA-1 hash of the commit."""
        ...

    @property
    def author(self) -> str:
        """Name of the original author of the commit."""
        ...

    @property
    def author_email(self) -> str:
        """Email address of the original author of the commit."""
        ...

    @property
    def author_time(self) -> int:
        """Timestamp (in seconds since epoch) when the commit was authored."""
        ...

    @property
    def committer(self) -> str:
        """Name of the person who committed the change (may differ from the author)."""
        ...

    @property
    def committer_email(self) -> str:
        """Email address of the committer."""
        ...

    @property
    def commit_time(self) -> int:
        """Timestamp (in seconds since epoch) when the commit was applied to the repository."""
        ...

    @property
    def message(self) -> str:
        """The full commit message."""
        ...

    @property
    def parents(self) -> List[str]:
        """List of parent commit hashes (used for merges and history tracking)."""
        ...


def get_commit_history(path: str) -> List[Commit]:
    """
    Retrieve the complete commit history of a Git repository.

    Parameters:
        path (str): Path to the local Git repository.

    Returns:
        List[Commit]: A list of Commit objects sorted by reverse chronological order (most recent first).

    Raises:
        IOError: If the repository cannot be opened or if reading commits fails.
    """
    ...


class DiffEntry:
    """
    Represents a summary of file changes between two commits.
    """

    @property
    def path(self) -> str:
        """The relative path of the file that changed."""
        ...

    @property
    def additions(self) -> int:
        """Number of lines added in the file between the two commits."""
        ...

    @property
    def deletions(self) -> int:
        """Number of lines deleted in the file between the two commits."""
        ...


def get_file_change_summary(path: str, commit1: str, commit2: str) -> List[DiffEntry]:
    """
    Compare two commits and return a summary of file-level changes (line additions and deletions).

    Parameters:
        path (str): Path to the Git repository.
        commit1 (str): SHA of the base commit (older).
        commit2 (str): SHA of the target commit (newer).

    Returns:
        List[DiffEntry]: A list of file summaries showing added/deleted line counts per file.

    Raises:
        IOError: If the repository path is invalid or commits cannot be found.
    """
    ...


class BlameLine:
    """
    Represents blame information for a single line in a file.

    Provides metadata about who last modified a line, when it was changed, and in which commit.
    """

    @property
    def line_number(self) -> int:
        """The 1-based line number in the file."""
        ...

    @property
    def content(self) -> str:
        """The actual content of the line."""
        ...

    @property
    def commit_hash(self) -> str:
        """The SHA of the commit that introduced or last modified the line."""
        ...

    @property
    def author(self) -> str:
        """The name of the original author of the line."""
        ...

    @property
    def author_email(self) -> str:
        """The email of the original author of the line."""
        ...

    @property
    def author_time(self) -> int:
        """Unix timestamp of when the line was authored."""
        ...

    @property
    def committer(self) -> str:
        """The name of the committer who last committed the line."""
        ...

    @property
    def commit_time(self) -> int:
        """Unix timestamp of when the line was committed."""
        ...

    @property
    def summary(self) -> str:
        """The commit message summary associated with the line."""
        ...


def get_file_blame(file_path: str) -> List[BlameLine]:
    """
    Retrieve blame information for each line in a file tracked by Git.

    Parameters:
        file_path (str): The path to the file (absolute, relative, or just the filename in the current directory).

    Returns:
        List[BlameLine]: A list of BlameLine objects with detailed commit attribution.

    Raises:
        IOError: If the file is not readable or not part of a Git repository.
        ValueError: If the file path cannot be resolved relative to the Git repository.
    """
    ...


def get_blame_for_files(file_paths: List[str]) -> Dict[str, List[BlameLine]]:
    """
    Perform parallel blame analysis on multiple files.

    Each file is analyzed concurrently using multiple threads for efficiency.
    The result is returned as a dictionary where the key is the file path,
    and the value is a list of BlameLine objects representing per-line attribution.

    Parameters:
        file_paths (List[str]): A list of file paths (absolute or relative) to analyze.

    Returns:
        Dict[str, List[BlameLine]]: A mapping from file paths to lists of BlameLine objects,
        each containing line-level commit attribution data.

    Raises:
        IOError: If a file cannot be accessed or read.
        ValueError: If the file path cannot be resolved relative to the Git repository.
        RuntimeError: If repository discovery or blame computation fails internally.
    """
    ...
