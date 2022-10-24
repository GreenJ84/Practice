var maximalSquare = function(matrix) {
    let area = 0;
    for(let i = 0; i < matrix.length; i++){
        for(let j = 0; j< matrix[0].length; j++){
            if (matrix[i][j] === '1'){
                let s = 1;
                let temp = 1;
                if (area === 0){
                area = 1;
                }
                while(i-s >= 0 && j-s >= 0){
                    if(matrix[i-s][j] === '1' & matrix[i-s][j-s] === '1' && matrix[i][j-s] === '1'){
                        if (s > 1){
                            let square = true;
                            let  y = i-s, x = j;
                            while (x >= j-s){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                x--;
                            }
                            while(y <= i){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                y++;
                            }
                        } else {
                            let side = s+1;
                            if (side*side > area){
                                area = side * side;
                            }
                            s++;
                        }
                    } else { break }
                }
                s = temp;
                while(i-s >= 0 && j+s < matrix[0].length){
                    if(matrix[i-s][j] === '1' & matrix[i-s][j+s] === '1' && matrix[i][j+s] === '1'){
                        if (s > 1){
                            let square = true;
                            let  y = i-s, x = j;
                            while (x <= j+s){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                x++;
                            }
                            while(y <= i){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                y++;
                            }
                        } else {
                            let side = s+1;
                            if (side*side > area){
                                area = side * side;
                            }
                            s++;
                        }
                    }  else { break }
                }
                s = temp;
                while(i+s < matrix.length && j+s < matrix[0].length){
                    if(matrix[i+s][j] === '1' & matrix[i+s][j+s] === '1' && matrix[i][j+s] === '1'){
                        if (s > 1){
                            let square = true;
                            let  y = i+s, x = j;
                            while (x <= j+s){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                x++;
                            }
                            while(y >= i){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                y--;
                            }
                        } else {
                            let side = s+1;
                            if (side*side > area){
                                area = side * side;
                            }
                            s++;
                        }
                    }  else { break }
                }
                s = temp;
                while(i+s < matrix.length && j-s >=0){
                    if(matrix[i+s][j] === '1' & matrix[i+s][j-s] === '1' && matrix[i][j-s] === '1'){
                        if (s > 1){
                            let square = true;
                            let  y = i+s, x = j;
                            while (x >= j-s){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                x--;
                            }
                            while(y >= i){
                                if (matrix[y][x] === '0'){
                                    square = false
                                }
                                y--;
                            }
                        } else {
                            let side = s+1;
                            if (side*side > area){
                                area = side * side;
                            }
                            s++;
                        }
                    }  else { break }
                }
            }
        }
    }
    console.log(area);
    return area
};

maximalSquare([["0","0","1","0"],["1","1","1","1"],["1","1","1","1"],["1","1","1","0"],["1","1","0","0"],["1","1","1","1"],["1","1","1","0"]]);