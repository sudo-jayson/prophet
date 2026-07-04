import time, random, playsound3, os

##Takes input from user and uses it to determine which script and shich sounds to use
def getProphet(question):
    if question == 1:
        return input("Who is speaking?\n")
    elif question == 2:
        return input("What sounds?\n")
    ##Default options for testing and development
    elif question == 3:
        return "dracula"
    elif question == 4:
        return "undertale"

##Instantiates the variables for script and sound effects and picks a random sound
textFile = os.getcwd() + '\\config\\' + getProphet(3) + '.txt'
sfxDir=os.getcwd() + '\\sfx\\' + getProphet(4)
sfxList = os.listdir(sfxDir)
sfx = sfxDir + "\\" + random.choice(sfxList)

##Gets location of text file and chooses a line at random from it
def getText(textFile):
    with open(textFile, 'r', encoding='utf-8') as f:
        lines = f.read().splitlines()
        return(random.choice(lines))

##Prints one letter at a time from the selected text and plays the sound effect 
def speak(text):
    for char in text:
        playsound3.playsound(sfx, block=False)
        print(char, end='', flush=True)
        time.sleep(getDelay())
    print()

##A random delay interval between each printing of letters
def getDelay():
    return random.uniform(0.05, 0.15)


print() ##Newlines for formatting
speak(getText(textFile))
print()