import time, random, playsound3, os, sys

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

##Gets location of text file and chooses a line at random from it
def getText(textFile):
    with open(textFile, 'r', encoding='utf-8') as f:
        lines = f.read().splitlines()
        return(random.choice(lines))

##Prints one letter at a time from the selected text and plays the sound effect 
def speak(text):
    time.sleep(.5)
    for char in text:
        sys.stdout.write(char)
        playsound3.playsound(sfx, block=False)
        sys.stdout.flush()
        time.sleep(random.uniform(minDelay, maxDelay))
    print()

print() ##Newlines for formatting
speak(getText(textFile))
print()