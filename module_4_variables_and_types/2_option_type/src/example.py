def find_index(target, names):
    for i in range(len(names)):
        if target == names[i]:
            return i
    # None is a special python value indicating `nothing`
    # Other languages may call this null.
    return None


def main():
    names = ["Kinan", "Matt", "Taishan", "Zach", "Kesar", "Lingie", "Emir"]
    grades = [ 0,      100,    95,        88,     99,      98,       97]

    # Find Matt's grade: this works!
    target = "Matt"
    i = find_index(target, names)
    if i is not None:
        print(grades[i])
    else:
        print('Not found')


    # Find Tom's grade.
    # find_index will return -1
    # But in Python, -1 is the index of the last element
    # This code will get confused and print 97, which is Emir's grade!
    target = "Tom"
    i = find_index(target, names)
    if i is not None:
        print(grades[i])
    else:
        print('Not found')

    # But, we have to REMEMBER to check for None.
    target = "Tom"
    i = find_index(target, names)
    print(grades[i])  # this errors!

main()