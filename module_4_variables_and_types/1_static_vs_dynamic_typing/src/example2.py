def find_index(target, names):
    for i in range(len(names)):
        if target == names[i]:
            return i
    return 'Not Found'


def main():
    names = ["Kinan", "Matt", "Taishan", "Zach", "Kesar", "Lingie", "Emir"]
    grades = [ 0,      100,    95,        88,     99,      98,       97]

    # Find Matt's grade: this works!
    target = "Matt"
    i = find_index(target, names)
    print(grades[i])


    # Find Tom's grade: there is no Tom, grades[i] will error!!!
    target = "Tom"
    i = find_index(target, names)
    print(grades[i])  # This line will error

main()