import nox


@nox.session(
    venv_backend="uv",
    python=["3.8", "3.9", "3.10", "3.11", "3.12", "3.13"],
    tags=["python-tests"],
)
def tests(session):
    """Run Python tests."""
    # Use uv with the specific Python version from the session
    python_version = f"{session.python}"
    session.run("uv", "sync", "--dev", "--python", python_version, external=True)
    session.run(
        "uv", "pip", "uninstall", "pdf2", "--python", python_version, external=True
    )
    session.run(
        "uv", "pip", "install", "-e", ".", "--python", python_version, external=True
    )
    session.run(
        "uv", "run", "--python", python_version, "pytest", "tests/", external=True
    )


@nox.session(tags=["rust-tests"])
def rust_tests(session):
    """Run Rust tests."""
    session.run("cargo", "test", external=True)


@nox.session(tags=["all-tests"])
def all_tests(session):
    """Run all tests (Python + Rust)."""
    rust_tests(session)
    tests(session)
