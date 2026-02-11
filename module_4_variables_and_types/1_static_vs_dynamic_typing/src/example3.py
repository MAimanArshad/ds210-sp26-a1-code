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
    if type(i) != int:
        print('Not found')
    else:
        print(grades[i])


    # Find Tom's grade: this works, but we have to remember
    # to explicitly check the type of i with an if statement!
    target = "Tom"
    i = find_index(target, names)
    if type(i) != int:
        print('Not found')
    else:
        print(grades[i])

main()