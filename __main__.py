import socket

from gui.gui import *

def createSocket():
    client=socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    client.connect(('localhost', 7070))
    
    return client    

def main():
    gui=Gui(createSocket())
    
if __name__ == "__main__":
    main()