def hello_emacs():
    return "Hello emacs!"

def test(message):
    return len(message)
if __name__ == "__main__":
    print(f"length: {test(str(input('message 입력: ')))}")
    print(hello_emacs)

