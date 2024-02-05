import tkinter as tk
import socket
from enum import Enum

class Opciones(Enum):
    MEDIO=1
    DIFICIL=2
    IA=3
    SALIR=-1
    
    def encodeOption(self) -> bytes:
        return self.value.to_bytes(1, byteorder='big', signed=True)

class Gui:
    socketCliente: socket.socket
    
    def __init__(self, socket):
        self.window = tk.Tk()
        self.button1 = tk.Button(self.window, text="Medio (16x16)")
        self.button2 = tk.Button(self.window, text="Dificil (30x16)")
        self.button3 = tk.Button(self.window, text="IA (30x16)")
        self.socketCliente=socket

    def createWindow(self):
        self.window.title("Buscaminas")
        self.window.geometry("1000x800")

        # Center the buttons
        self.button1.place(relx=0.5, rely=0.2, anchor="center")
        self.button2.place(relx=0.5, rely=0.5, anchor="center")
        self.button3.place(relx=0.5, rely=0.8, anchor="center")
        
        self.button1.config(command=self.medio)
        self.button2.config(command=self.dificil)
        self.button3.config(command=self.ia)

        self.window.protocol("WM_DELETE_WINDOW", self.on_closing)  # Bind the function to window close event

        self.window.mainloop()
    
    def medio(self):
        self.socketCliente.send(Opciones.MEDIO.encodeOption())
        
    def dificil(self):
        self.socketCliente.send(Opciones.DIFICIL.encodeOption())
        
    def ia(self):
        self.socketCliente.send(Opciones.IA.encodeOption()) 

    def on_closing(self):
        self.socketCliente.send(Opciones.SALIR.encodeOption())  
        self.window.destroy()

    