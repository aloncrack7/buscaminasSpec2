use rand::Rng;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum EstadoPartida{
    Jugando,
    Ganada,
    Perdida,
    SinIniciar
}

pub struct Tablero{
    filas: isize,
    columnas: isize,
    minas: i32,
    tablero: Vec<Vec<i32>>,
    tablero_visible: Vec<Vec<i32>>,
    minas_cercanas: Vec<Vec<i32>>,
    minas_encontradas: i32,
    casillas_descubiertas: i32,
    estado: EstadoPartida,
}

impl Tablero{
    pub fn new(filas: usize, columnas: usize, minas: i32) -> Self{
        let mut tablero = Tablero{
            filas: filas as isize,
            columnas: columnas as isize,
            minas: minas,
            tablero: vec![vec![0; columnas]; filas],
            tablero_visible: vec![vec![0; columnas]; filas],
            minas_cercanas: vec![vec![0; columnas]; filas],
            minas_encontradas: 0,
            casillas_descubiertas: 0,
            estado: EstadoPartida::SinIniciar,
        };
        tablero.generar_tablero();
        tablero.generar_minas_cercanas();

        tablero
    }

    pub fn get_estado(&self) -> EstadoPartida{
        self.estado
    }

    fn generar_tablero(&mut self){
        let mut rng = rand::thread_rng();
        let mut minas_generadas = 0;
        while minas_generadas < self.minas{
            let fila = rng.gen_range(0..self.filas);
            let columna = rng.gen_range(0..self.columnas);
            if self.tablero[fila as usize][columna as usize] != -1{
                self.tablero[fila as usize][columna as usize] = -1;
                minas_generadas += 1;
            }
        }
    }

    fn generar_minas_cercanas(&mut self){
        for i in 0..self.filas{
            for j in 0..self.columnas{
                if self.tablero[i as usize][j as usize] == -1{
                    for k in -1..2{
                        for l in -1..2{
                            if i!=0 || k!=0{
                                let fila: isize = i+k;
                                let columna: isize = j+l;

                                if fila>= 0 && fila<self.filas && columna>=0 && columna<self.columnas{
                                    if self.tablero[fila as usize][columna as usize] == -1{
                                        self.tablero[fila as usize][columna as usize] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn descubrir_casilla(&mut self, fila: usize, columna: usize){
        if self.estado.eq(&EstadoPartida::SinIniciar){
            self.descubrir_casilla_primera_ronda(fila, columna);
        }else{
            if self.tablero[fila][columna] == -1{
                self.estado = EstadoPartida::Perdida;
            }
        }

        self.hacer_visible(fila, columna);
    }

    fn descubrir_casilla_primera_ronda(&mut self, fila: usize, columna: usize){
        self.estado = EstadoPartida::Jugando;

        if self.tablero[fila][columna] == -1{
            self.tablero[fila][columna] = 0;
            for i in -1..2{
                for j in -1..2{
                    if i!=0 || j!=0{
                        let fila: isize = (fila as isize)+i;
                        let columna: isize = (columna as isize)+j;

                        if fila>=0 && fila<self.filas && columna>=0 && columna<self.columnas{
                            if self.tablero[fila as usize][columna as usize]!=-1{
                                self.tablero[fila as usize][columna as usize]-=1;
                            }
                        }
                    }
                }
            }
        }

        let mut colocado=false;
        while !colocado {
            let mut rng = rand::thread_rng();
            let fila = rng.gen_range(0..self.filas);
            let columna = rng.gen_range(0..self.columnas);
            if self.tablero[fila as usize][columna as usize]!=-1{
                colocado=true;

                self.tablero[fila as usize][columna as usize] = -1;

                for i in -1..2{
                    for j in -1..2{
                        if i!=0 || j!=0{
                            let fila: isize = (fila as isize)+i;
                            let columna: isize = (columna as isize)+j;

                            if fila>=0 && fila<self.filas && columna>=0 && columna<self.columnas{
                                if self.tablero[fila as usize][columna as usize]==-1{
                                    self.tablero[fila as usize][columna as usize]+=1;
                                }
                            }
                        }
                    }
                } 
            }
        }
    }

    fn hacer_visible(&mut self, fila: usize, columna: usize){
        self.tablero_visible[fila][columna] = 1;
        self.casillas_descubiertas += 1;

        if self.tablero[fila][columna]==0{
            for i in -1..2{
                for j in -1..2{
                    if i!=0 || j!=0{
                        let fila: isize = (fila as isize)+i;
                        let columna: isize = (columna as isize)+j;

                        if fila>=0 && fila<self.filas && columna>=0 && columna<self.columnas{
                            if self.tablero_visible[fila as usize][columna as usize] == 0{
                                self.hacer_visible(fila as usize, columna as usize);
                            }
                        }
                    }
                }
            }
        }else if self.estado.eq(&EstadoPartida::Perdida){
            for i in 0..self.filas{
                for j in 0..self.columnas{
                    let i = i as usize;
                    let j = j as usize;

                    if self.tablero[i][j]==-1{
                        self.tablero_visible[i][j]=1;
                    }
                }
            }
        }else {
            if self.casillas_descubiertas==(self.filas*self.columnas) as i32 -self.minas{
                self.estado=EstadoPartida::Ganada;
            }
        }
    }
}