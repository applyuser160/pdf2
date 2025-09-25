import nox


@nox.session(tags=["rust-tests"])
def rust_tests(session):
    """Run Rust tests."""
    session.run("cargo", "test", external=True)


@nox.session(tags=["all-tests"])
def all_tests(session):
    """Run all tests (Rust)."""
    rust_tests(session)
