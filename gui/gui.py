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
    def __init__(self, socket):
        self.window = tk.Tk()
        self.socketCliente=socket
        self.buttons=[]

        self.difficultyWindow()

    def difficultyWindow(self):
        self.window.title("Buscaminas: Seleccionar Dificultad")
        self.window.geometry("1000x800")

        # Center the buttons
        button1=tk.Button(self.window, text="Medio (16x16)", command=self.medio)
        button1.place(relx=0.5, rely=0.2, anchor="center")

        button2=tk.Button(self.window, text="Dificil (30x16)", command=self.dificil)
        button2.place(relx=0.5, rely=0.5, anchor="center")
        
        button3=tk.Button(self.window, text="IA (30x16)", command=self.ia)
        button3.place(relx=0.5, rely=0.8, anchor="center")

        self.window.protocol("WM_DELETE_WINDOW", self.on_closing)  # Bind the function to window close event
        self.window.mainloop()


    def medio(self):
        self.socketCliente.send(Opciones.MEDIO.encodeOption())
        self.createBoard(16, 16)
        
    def dificil(self):
        self.socketCliente.send(Opciones.DIFICIL.encodeOption())
        self.createBoard(30, 16)
        
    def ia(self):
        self.socketCliente.send(Opciones.IA.encodeOption()) 
        self.createBoard(30, 16)

    def on_closing(self):
        self.socketCliente.send(Opciones.SALIR.encodeOption())  
        self.window.destroy()
    
    def createBoard(self, numFilas, numColumnas):
        # Cerrar la ventana de selección de dificultad
        self.window.destroy()

        # Crear la ventana del juego con el tablero de botones
        self.game_window = tk.Tk()
        self.game_window.title("Buscaminas")
        self.game_window.geometry("1000x800")

        btns_frame=tk.Frame(self.game_window)
        btns_frame.pack()
        for i in range(numFilas):
            row_buttons = []
            for j in range(numColumnas):
                btn = tk.Button(btns_frame, width=2, height=1, command=lambda fila=i, columna=j: self.button_click(fila, columna))
                btn.grid(row=i+1, column=j)
                btn.bind("<Button-3>", lambda event, fila=i, columna=j: self.right_click(event, fila, columna)) 
                row_buttons.append(btn)
            self.buttons.append(row_buttons)

        self.game_window.protocol("WM_DELETE_WINDOW", self.on_game_window_closing)
        self.game_window.mainloop()

    def button_click(self, row, col):
        print(f"Botón izquierdo en la posición ({row}, {col})")
        opcion=0
        self.socketCliente.send(opcion.to_bytes(1, byteorder='big', signed=True))
        self.socketCliente.send(row.to_bytes(1, byteorder='big', signed=False))
        self.socketCliente.send(col.to_bytes(1, byteorder='big', signed=False))

        numDescubiertos=int.from_bytes(self.socketCliente.recv(1), byteorder='big', signed=False)
        print(f"numDescubiertos: {numDescubiertos}")

        for i in range(0, numDescubiertos):
            fila = int.from_bytes(self.socketCliente.recv(1), byteorder='big', signed=False)
            columna = int.from_bytes(self.socketCliente.recv(1), byteorder='big', signed=False)
            valor = int.from_bytes(self.socketCliente.recv(1), byteorder='big', signed=True)

            self.buttons[fila][columna]['text']=str(valor)

            print(f"Fila: {fila}")
            print(f"Columna: {columna}")
            print(f"Valor: {valor}")

    def right_click(self, event, row, col):
        print(f"Botón derecho en la posición ({row}, {col})")
        opcion=1
        self.socketCliente.send(opcion.to_bytes(1, byteorder='big', signed=True))
        self.socketCliente.send(row.to_bytes(1, byteorder='big', signed=False))
        self.socketCliente.send(col.to_bytes(1, byteorder='big', signed=False))

    def on_game_window_closing(self):
        self.socketCliente.send(Opciones.SALIR.encodeOption())  
        self.game_window.destroy()