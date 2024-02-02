import os
import platform
import socket

from gui.gui import *

# def createSocket():
#     s=socket.socket(socket.AF_INET, socket.SOCK_STREAM)
#     s.bind(('localhost', 7070))
    

def main():
    if platform.system() == "Windows":
        os.system(".\\buscaminasBackend\\target\\release\\buscaminasBackend.exe")
    else:
        os.system("./buscaminasBackend/target/release/buscaminasBackend")
        
    createWindow()
    
if __name__ == "__main__":
    main()