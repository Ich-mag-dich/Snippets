def test(message):
    return len(message)
if __name__ == "__main__":
    print(f"length: {test(str(input('message 입력: ')))}")

