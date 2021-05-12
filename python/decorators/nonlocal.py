x = 1

def func1():
    x = 2
    
    def func2():
        #x = 3
        #nonlocal x
        #global x
        print('func2', x)

    func2()
    print('func1', x)

func1()
print(x)

#func2 2
#func1 2
#1

#func2 3
#func1 2
#1

#func2 3
#func1 3
#1

#func2 3
#func1 2
#3
