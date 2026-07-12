import time, random, playsound3, os

##Takes input from user and uses it to determine which script and shich sounds to use
def getProphet(question):
    if question == 1:
        return input("Who is speaking?\n")
    ##Temp default option for testing
    else:   return "test"

##Instantiates the variables for script and sound effects based on config file
configFile = os.getcwd() + '\\config\\' + getProphet(3) + '.conf'
with open(configFile, 'r') as f:
    data = f.read().splitlines()
    textFile = os.getcwd() + '\\config\\' + data[0] +'.txt'
    sfxDir = os.getcwd() + '\\sfx\\' +data[1]
    minDelay = float(data[2])
    maxDelay = float(data[3])

##Gets list of sounds and picks one at random
sfxList = os.listdir(sfxDir)
sfx = sfxDir + "\\" + random.choice(sfxList)

sfxName= sfx.split("_")
print(sfxName[1])

##Gets location of text file and chooses a line at random from it
def getText(textFile):
    with open(textFile, 'r', encoding='utf-8') as f:
        lines = f.read().splitlines()
        return(random.choice(lines))

##Prints one letter at a time from the selected text and plays the sound effect 
def speak(text):
    time.sleep(maxDelay)
    for char in text:
        print(char, end='', flush=True)
        playsound3.playsound(sfx, block=False)
        time.sleep(random.uniform(minDelay, maxDelay))
    print()

print() ##Newlines for formatting
speak(getText(textFile))
print()