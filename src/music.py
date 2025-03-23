from pygame import mixer
def new():
    mixer.init()
    mixer.Channel(1)
    mixer.Channel(1).play(mixer.Sound("E:/coding/programmes/rust/demo/assets/AudioClip/Angel24 - Cotton Candy Island.wav"),-1)
    
def init():
    mixer.init()
    
def new_channel():
    i = mixer.get_num_channels()
    mixer.Channel(i+1)
    return i+1

def play(file, channel, loop):
    mixer.Channel(channel).play(mixer.Sound(file),loop)

def pause(channel):
    mixer.Channel(channel).pause()
    
def unpause(channel):
    mixer.Channel(channel).unpause()
    
# new()
# while (True):
#     pass