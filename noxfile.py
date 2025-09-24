import nox


@nox.session(
    python=["3.8", "3.9", "3.10", "3.11", "3.12", "3.13"], tags=["python-tests"]
)
def tests(session):
    """Run Python tests."""
    session.run("uv", "sync", "--dev", external=True)
    session.run("uv", "pip", "install", "-e", ".", external=True)
    session.run("uv", "run", "pytest", "tests/", external=True)


@nox.session(tags=["rust-tests"])
def rust_tests(session):
    """Run Rust tests."""
    session.run("cargo", "test", external=True)


@nox.session(tags=["all-tests"])
def all_tests(session):
    """Run all tests (Python + Rust)."""
    rust_tests(session)
    tests(session)
