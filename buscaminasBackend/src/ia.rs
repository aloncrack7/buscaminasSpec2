pub struct TabeleroInterno {
    filas: isize,
    columnas: isize,
    tablero: Vec<Vec<i8>>,
    buscaminas: buscaminas::Tablero
}

impl TabeleroInterno {
    pub fn new(filas: usize, columnas: usize, tablero_visible: Vec<Vec<i8>>, buscaminas: buscaminas::Tablero) -> TabeleroInterno {
        let mut tablero = vec![];
        for i in 0..filas {
            let mut fila = vec![];
            for j in 0..columnas {
                fila.push(i);
            }
            tablero.push(fila);
        }

        TabeleroInterno {
            filas: filas as isize,
            columnas: columnas as isize,
            tablero,
            buscaminas
        }
    }

    pub fn getTablero(&self) -> &Vec<Vec<i8>> {
        &self.tablero
    }

    pub fn nextMove(&self) -> Vec<(usize, usize, i8)> {
        for i in 0..self.filas {
            for j in 0..self.columnas {
                let fila = i as usize;
                let columna = j as usize;

                if self.tablero[i as usize][j as usize]>0 {
                    nextMoveDiscoverBombs(fila, columna);
                }
            }
        }
    }

    fn nextMoveDiscoverBombs(&self, fila: usize, columna: usize) -> Vec<(isize, isize, i8)> {
        let mut contadorBanderas=0;
        let mut casillasAMarcar=vec![];
        
        for i in -1..2 {
            for j in -1..2 {
                if i!=0 || j!=0 {
                    let fila: isize = (fila as isize)+i;
                    let columna: isize = (columna as isize)+j;

                    if fila>=0 && fila<self.filas && columna>=0 && columna<self.columnas {
                        let fila = fila as usize;
                        let columna = columna as usize;

                        if self.tablero[fila][columna]==-2 {
                            contadorBanderas+=1;
                            casillasAMarcar.push((fila, columna));
                        }else if self.tablero[fila][columna]==-1 {
                            contadorBanderas+=1;
                        }
                    }
                }
            }
        }

        let mut resultado=vec![];
        if contadorBanderas==self.tablero[fila][columna] {
            for (fila, columna) in casillasAMarcar {
                resultado.push(buscaminas.descubrir_casilla(fila, columna));
            }
        }

        resultado
    }
}