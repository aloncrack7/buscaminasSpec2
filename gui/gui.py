import tkinter as tk

window = tk.Tk()

button1 = tk.Button(window, text="Medio (16x16)")
button2 = tk.Button(window, text="Dificil (30x16)")
button3 = tk.Button(window, text="IA (30x16)")

def createWindow():
    window.title("Buscaminas")
    window.geometry("1000x800")

    # Center the buttons
    button1.place(relx=0.5, rely=0.2, anchor="center")
    button2.place(relx=0.5, rely=0.5, anchor="center")
    button3.place(relx=0.5, rely=0.8, anchor="center")
    
    button1.config(command=facil)
    button2.config(command=dificil)
    button3.config(command=ia)

    window.mainloop()

def facil():
    print("Facil")
    
def dificil():
    print("Dificil")
    
def ia():
    print("IA")
    
def setSocket(socket):
    print("Set socket")

    