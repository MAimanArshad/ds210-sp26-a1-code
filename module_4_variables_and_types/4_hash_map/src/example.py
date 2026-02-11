def main():
    names = ["Kinan", "Matt", "Taishan", "Zach", "Kesar", "Lingie", "Emir"]
    grades = [ 0,      100,    95,        88,     99,      98,       97]

    # in Python, a HashMap is called dictonary (or dict)
    map = dict()  # or equivalently, map = {}
    for i in range(len(names)):
        map[names[i]] = grades[i]

    target = "Matt"
    grade = map[target]
    print(grade)
    
    target = "Kesar"
    grade = map[target]
    print(grade)
    
    
    # if we access the map with a name (i.e. key) that does not exist,
    # in python, it errors!
    # we need to check if it exists first.
    target = "Tom"
    if target in map:
        grade = map[target]
        print(grade)
    else:
        print("Not found")


    # To demonstrate the error:
    target = "Tom"
    grade = map[target]  # This produces an error!
    print(grade)

main()
