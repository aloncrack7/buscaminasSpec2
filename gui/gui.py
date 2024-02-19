import tkinter as tk
import requests
from enum import Enum

class Opciones(Enum):
    MEDIO=1
    DIFICIL=2
    IA=3
    SALIR=-1
    
    def encodeOption(self) -> bytes:
        return self.value.to_bytes(1, byteorder='big', signed=True)

class Gui:
    def __init__(self):
        self.window = tk.Tk()
        self.base_url = "http://localhost:7070"
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
        if requests.get(f"{self.base_url}/medio").status_code == 200:
            self.createBoard(16, 16)
            print("Conectado al servidor")
        else:
            print("No se pudo conectar al servidor")
        
    def dificil(self):
        if requests.get(f"{self.base_url}/dificil").status_code == 200:
            self.createBoard(30, 16)
            print("Conectado al servidor")
        else:
            print("No se pudo conectar al servidor")
        
    def ia(self):
        if requests.get(f"{self.base_url}/ia").status_code == 200:
            self.createBoard(30, 16)
            print("Conectado al servidor")
        else:
            print("No se pudo conectar al servidor")

    def on_closing(self):
        if requests.get(f"{self.base_url}/salir").status_code == 200:
            self.window.destroy()
        else:
            print("No se pudo conectar al servidor")  
    
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
                btn.bind("<Button-2>", lambda event, fila=i, columna=j: self.center_click(event, fila, columna))
                btn['text'] = ""
                row_buttons.append(btn)
            self.buttons.append(row_buttons)

        self.game_window.protocol("WM_DELETE_WINDOW", self.on_game_window_closing)
        self.game_window.mainloop()

    def button_click(self, row, col):
        if self.buttons[row][col]['text']=="" or self.buttons[row][col]['text']==" ":
            print(f"Botón izquierdo en la posición ({row}, {col})")
            payload = {"fila": row, "columna": col, "opcion": "seleccionar"}

            response = requests.get(f"{self.base_url}/jugar", params=payload)
            
            if response.status_code == 200:
                data = response.json()
                for casilla in data:
                    fila = int(casilla[0])
                    columna = int(casilla[1])
                    valor = casilla[2]

                    if valor == -1:
                        if self.buttons[fila][columna]['text']!="🚩":
                            self.buttons[fila][columna]['text']="💣"
                            
                            if fila==row and columna==col:
                                self.buttons[fila][columna]['foreground']='black'
                                self.buttons[fila][columna]['background']='red'
                            else:
                                self.buttons[fila][columna]['foreground']='black'
                                self.buttons[fila][columna]['background']='white'
                            # self.buttons[fila][columna].config(state="disabled")
                    elif valor == 0:
                        self.buttons[fila][columna]['text']=" "
                        self.buttons[fila][columna]['foreground']='white'
                        self.buttons[fila][columna]['background']='dark gray'
                        # self.buttons[fila][columna].config(state="disabled")
                    else:
                        self.buttons[fila][columna]['text']=str(valor)
                        if valor==1:
                            color='blue'
                        elif valor==2:
                            color='green'
                        elif valor==3:
                            color='red'
                        elif valor==4:
                            color='dark blue'
                        elif valor==5:        
                            color='dark red'
                        elif valor==6:
                            color='cyan'
                        elif valor==7:
                            color='black'
                        elif valor==8:
                            color='light grey'
                        else:
                            color='black'
                        self.buttons[fila][columna]['foreground']='black'
                        self.buttons[fila][columna]['background']=color
                        # self.buttons[fila][columna].config(state="disabled")    
                        
            else:
                print("No se pudo conectar al servidor")

    def right_click(self, _, row, col):
        if self.buttons[row][col]['text']=="🚩" or self.buttons[row][col]['text']=="":
            print(f"Botón derecho en la posición ({row}, {col})")
            payload = {"fila": row, "columna": col, "opcion": "bandera"}

            response = requests.post(f"{self.base_url}/jugar", params=payload)
            
            if self.buttons[row][col]['text']!="🚩":
                self.buttons[row][col]['foreground']='black'
                self.buttons[row][col]['background']='red'
                self.buttons[row][col]['text']="🚩"
            else:
                self.buttons[row][col]['foreground']='black'
                self.buttons[row][col]['background']='white'
                self.buttons[row][col]['text']=" "
            
            if response.status_code == 200:
                print("Bandera puesta")
            else:
                print("No se pudo conectar al servidor")
                
    def center_click(self, _, row, col):
        print(f"Botón central en la posición ({row}, {col})")
        if self.buttons[row][col]['text']!="" or self.buttons[row][col]['text']!=" " \
            and self.buttons[row][col]['text']!="🚩":
            print(f"Botón central en la posición ({row}, {col})")
            payload = {"fila": row, "columna": col, "opcion": "seleccionarVarios"}

            response = requests.get(f"{self.base_url}/jugar", params=payload)
            
            if response.status_code == 200:
                data = response.json()
                for casilla in data:
                    fila = int(casilla[0])
                    columna = int(casilla[1])
                    valor = casilla[2]

                    if valor == -1:
                        if self.buttons[fila][columna]['text']!="🚩":
                            self.buttons[fila][columna]['text']="💣"                           
                            if fila==row and columna==col:
                                self.buttons[fila][columna]['foreground']='black'
                                self.buttons[fila][columna]['background']='red'
                            else:
                                self.buttons[fila][columna]['foreground']='black'
                                self.buttons[fila][columna]['background']='white'
                            self.buttons[fila][columna].config(state="disabled")
                    elif valor == 0:
                        self.buttons[fila][columna]['text']=" "
                        self.buttons[fila][columna]['foreground']='white'
                        self.buttons[fila][columna]['background']='dark gray'
                        self.buttons[fila][columna].config(state="disabled")
                    else:
                        self.buttons[fila][columna]['text']=str(valor)
                        if valor==1:
                            color='blue'
                        elif valor==2:
                            color='green'
                        elif valor==3:
                            color='red'
                        elif valor==4:
                            color='dark blue'
                        elif valor==5:        
                            color='dark red'
                        elif valor==6:
                            color='cyan'
                        elif valor==7:
                            color='black'
                        elif valor==8:
                            color='light grey'
                        else:
                            color='black'
                        self.buttons[fila][columna]['foreground']='black'
                        self.buttons[fila][columna]['background']=color
                        self.buttons[fila][columna].config(state="disabled")    
                        
            else:
                print("No se pudo conectar al servidor")

    def on_game_window_closing(self):
        if requests.get(f"{self.base_url}/salir").status_code == 200:
            self.game_window.destroy()
        else:
            print("No se pudo conectar al servidor") 