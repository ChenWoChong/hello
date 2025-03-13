import nox

nox.options.sessions = ["test"]


@nox.session
def test(session: nox.Session):
    session.env["MATURIN_PEP517_ARGS"] = "--profile=dev"
    session.install("maturin")
    # 以可编辑模式安装当前包（包含 Rust 扩展）
    session.run("maturin", "develop", "-r")

    # 安装测试依赖
    session.install("pytest", "pytest-benchmark")

    # 设置 PYTHONPATH（关键步骤！）
    session.env["PYTHONPATH"] = "."
    session.run("pytest")


@nox.session
def bench(session: nox.Session):
    session.env["MATURIN_PEP517_ARGS"] = "--profile=dev"
    session.install(".[dev]")
    session.run("pytest", "--benchmark-enable")
