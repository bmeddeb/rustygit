class Repo:
    """
    A Python-friendly Git repository handler powered by Rust for performance and parallelism.
    """

    def __init__(self, path: str):
        """
        Open an existing repository.

        Parameters:
            path (str): Path to the existing repository.

        Raises:
            IOError: If the repository at the given path cannot be opened.
        """
        ...

    @staticmethod
    def init(path: str) -> 'Repo':
        """
        Initialize a new Git repository.

        Parameters:
            path (str): Directory path where the repository will be initialized.

        Returns:
            Repo: A new Repo instance pointing to the initialized repository.

        Raises:
            IOError: If initialization fails.
        """
        ...


@staticmethod
def clone(url: str, path: Optional[str] = None, username: Optional[str] = None, token: Optional[str] = None) -> 'Repo':
    """
    Clone a Git repository from a URL with optional authentication.

    Parameters:
        url (str): URL of the repository to clone.
        path (Optional[str]): Local path for the repository (defaults to repository name).
        username (Optional[str]): GitHub username for authentication (required for private repositories).
        token (Optional[str]): Personal Access Token for GitHub authentication.

    Returns:
        Repo: Instance pointing to the cloned repository.

    Raises:
        IOError: If cloning fails due to authentication or other issues.
    """
    ...

    @property
    def path(self) -> str:
        """
        Get the local filesystem path of the repository.

        Returns:
            str: Path to the repository.
        """
        ...

    def is_bare(self) -> bool:
        """
        Check if the repository is a bare repository.

        Returns:
            bool: True if the repository is bare, False otherwise.
        """
        ...

    def fetch_updates(self, remote_name: Optional[str] = 'origin', branch: Optional[str] = 'main') -> None:
        """
        Fetch updates from a remote repository.

        Parameters:
            remote_name (Optional[str]): Name of the remote to fetch from. Defaults to 'origin'.
            branch (Optional[str]): Branch to fetch. Defaults to 'main'.

        Raises:
            Exception: If fetching updates fails.
        """
        ...

    def list_remotes(self) -> List[str]:
        """
        List all remotes configured for the repository.

        Returns:
            List[str]: List of remote names.

        Raises:
            Exception: If listing remotes fails.
        """
        ...

    def status(self) -> List[str]:
        """
        Get the repository's current status (changed, new, or deleted files).

        Returns:
            List[str]: List of paths for files with changes.

        Raises:
            Exception: If retrieving status fails.
        """
        ...

    @staticmethod
    async def async_clone(
        urls: List[str],
        base_dir: Optional[str] = ".",
        username: Optional[str] = None,
        token: Optional[str] = None
    ) -> List[str]:
        """
        Asynchronously clone multiple Git repositories concurrently.

        Parameters:
            urls (List[str]): A list of Git repository URLs to clone.
            base_dir (Optional[str]): Base directory to clone repositories into.
                                      Each repository will be placed in its own subdirectory
                                      named after the repository (default is current directory).
            username (Optional[str]): GitHub username for authentication (required for private repositories).
            token (Optional[str]): Personal Access Token for GitHub authentication.

        Returns:
            List[str]: A list of paths to the successfully cloned repositories.

        Raises:
            Exception: If any of the clone operations fail. Partial results may be returned.
        """
        ...
