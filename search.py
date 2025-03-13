def search_py(contents: str, needle: str) -> int:
    total = 0
    for line in contents.splitlines():
        for word in line.split(" "):
            if word == needle:
                total += 1
    return total
