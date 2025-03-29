import rustygit
import os
import shutil
import datetime


def main():
    print("RustyGit Basic Example")
    print("======================")

    # Example 1: Clone with an explicit path
    # Create a unique directory name with timestamp
    unique_dir = f"./rustygit-clone-{datetime.datetime.now().strftime('%Y%m%d%H%M%S')}"

    # Make sure the directory doesn't exist
    if os.path.exists(unique_dir):
        shutil.rmtree(unique_dir)

    # Clone a repository with explicit path
    print(f"Example 1: Cloning repository to {unique_dir}...")
    repo1 = rustygit.Repo.clone(
        "https://github.com/bmeddeb/rustygit.git",
        unique_dir
    )
    print(f"Repository cloned at {repo1.path}")
    print(f"Is bare repository: {repo1.is_bare()}")

    # Example 2: Clone with default path
    # Let's determine what the default path would be for demo purposes
    default_path = "./rustygit"

    # Clean up if it already exists
    if os.path.exists(default_path):
        shutil.rmtree(default_path)

    # Clone a repository with default path (None)
    print("\nExample 2: Cloning repository with default path...")
    repo2 = rustygit.Repo.clone(
        "https://github.com/bmeddeb/rustygit.git"
    )
    print(f"Repository cloned at {repo2.path}")
    print(f"Is bare repository: {repo2.is_bare()}")

    # Clean up the cloned repositories
    print("\nCleaning up...")
    if os.path.exists(unique_dir):
        shutil.rmtree(unique_dir)
    if os.path.exists(default_path):
        shutil.rmtree(default_path)
    print("Done!")


if __name__ == "__main__":
    main()
