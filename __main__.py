import os
import platform

def main():
    if platform.system() == "Windows":
        os.system(".\\buscaminasBackend\\target\\release\\buscaminasBackend.exe")
    else:
        os.system("./buscaminasBackend/target/release/buscaminasBackend")
    
if __name__ == "__main__":
    main()