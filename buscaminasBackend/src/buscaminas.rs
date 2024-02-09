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
    tablero: Vec<Vec<i8>>,
    tablero_visible: Vec<Vec<bool>>,
    casillas_descubiertas: i32,
    estado: EstadoPartida,
}

impl std::fmt::Display for Tablero {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for fila in &self.tablero{
            for casilla in fila{
                write!(f, "{}", casilla)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Tablero{
    pub fn new(filas: usize, columnas: usize, minas: i32) -> Self{
        let mut tablero = Tablero{
            filas: filas as isize,
            columnas: columnas as isize,
            minas: minas,
            tablero: vec![vec![0; columnas]; filas],
            tablero_visible: vec![vec![false; columnas]; filas],
            casillas_descubiertas: 0,
            estado: EstadoPartida::SinIniciar,
        };
        tablero.generar_tablero();

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
                if self.tablero[i as usize][j as usize] != -1{
                    for k in -1..2{
                        for l in -1..2{
                            if !(k==0 && l==0){
                                let fila: isize = i+k;
                                let columna: isize = j+l;

                                if fila>= 0 && fila<self.filas && columna>=0 && columna<self.columnas{
                                    if self.tablero[fila as usize][columna as usize] == -1{
                                        self.tablero[i as usize][j as usize] += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn descubrir_casilla(&mut self, fila: usize, columna: usize) -> Vec<(u8, u8, i8)>{
        if self.estado.eq(&EstadoPartida::SinIniciar){
            self.descubrir_casilla_primera_ronda(fila, columna);
        }else{
            if self.tablero[fila][columna] == -1{
                self.estado = EstadoPartida::Perdida;
            }
        }

        self.hacer_visible(fila, columna)
    }

    fn descubrir_casilla_primera_ronda(&mut self, fila: usize, columna: usize){
        self.estado = EstadoPartida::Jugando;

        if self.tablero[fila][columna] == -1{
            self.tablero[fila][columna] = 0;

            let mut colocado=false;
            while !colocado {
                let mut rng = rand::thread_rng();
                let filaAntes=fila;
                let fila = rng.gen_range(0..self.filas) as usize;
                let columnaAntes=columna;
                let columna = rng.gen_range(0..self.columnas) as usize;
                if (fila!=filaAntes && columna!=columnaAntes) && self.tablero[fila][columna]!=-1{
                    colocado=true;
                    self.tablero[fila][columna] = -1;
                }
            }
        }

        self.generar_minas_cercanas();
    }

    fn hacer_visible(&mut self, fila: usize, columna: usize) -> Vec<(u8, u8, i8)>{
        if self.tablero_visible[fila][columna]{
            return vec![];
        }

        let mut resultado=vec![(fila as u8, columna as u8, self.tablero[fila][columna])];

        self.tablero_visible[fila][columna] = true;
        self.casillas_descubiertas += 1;

        if self.tablero[fila][columna]==0{
            for i in -1..2{
                for j in -1..2{
                    if i!=0 || j!=0{
                        let fila: isize = (fila as isize)+i;
                        let columna: isize = (columna as isize)+j;

                        if fila>=0 && fila<self.filas && columna>=0 && columna<self.columnas{
                            resultado.append(&mut self.hacer_visible(fila as usize, columna as usize));
                        }
                    }
                }
            }
        }else if self.estado.eq(&EstadoPartida::Perdida){
            let mut resultado: Vec<(u8, u8, i8)>=vec![];

            for i in 0..self.filas{
                for j in 0..self.columnas{
                    if !self.tablero_visible[i as usize][j as usize]{
                        resultado.push((i as u8, j as u8, self.tablero[i as usize][j as usize]));
                    }
                }
            }

            return resultado;
        }else if self.casillas_descubiertas==(self.filas*self.columnas) as i32 -self.minas{
            self.estado=EstadoPartida::Ganada;

            for i in 0..self.filas{
                for j in 0..self.columnas{
                    if !self.tablero_visible[i as usize][j as usize]{
                        resultado.push((i as u8, j as u8, self.tablero[fila][columna]));
                    }
                }
            }
        }

        resultado
    }
}